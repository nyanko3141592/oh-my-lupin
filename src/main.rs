use ab_glyph::{Font, FontRef, ScaleFont};
use anyhow::{anyhow, Result};
use clap::Parser;
use image::{GrayImage, Luma};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "macos")]
const DEFAULT_FONT: &str = "/System/Library/Fonts/Helvetica.ttc";
#[cfg(target_os = "linux")]
const DEFAULT_FONT: &str = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
#[cfg(target_os = "windows")]
const DEFAULT_FONT: &str = "C:\\Windows\\Fonts\\arial.ttf";

#[derive(Parser)]
#[command(name = "oh-my-lupin")]
#[command(about = "Convert text to braille using any font", long_about = None)]
struct Cli {
    #[arg(help = "Text to display")]
    text: String,

    #[arg(help = "Font size in pixels", default_value = "50")]
    font_size: u32,

    #[arg(short, long, help = "Path to font file (TTF/OTF)")]
    font: Option<String>,

    #[arg(
        short,
        long,
        default_value = "128",
        help = "Threshold for dot rendering (0-255, default: 128)"
    )]
    threshold: u8,

    #[arg(long, help = "Disable animation")]
    no_animate: bool,

    #[arg(
        short,
        long,
        default_value = "200",
        help = "Animation delay in milliseconds (default: 200)"
    )]
    delay: u64,
}

struct FontToDots;

impl FontToDots {
    fn text_to_dots(
        font_path: &str,
        text: &str,
        font_size: u32,
        threshold: u8,
    ) -> Result<Vec<Vec<bool>>> {
        if !Path::new(font_path).exists() {
            return Err(anyhow!("Font file not found: {}", font_path));
        }

        let font_data = fs::read(font_path)?;
        let font = FontRef::try_from_slice(&font_data)?;

        let scale = ab_glyph::PxScale {
            x: font_size as f32,
            y: font_size as f32,
        };
        let scaled_font = font.as_scaled(scale);

        let margin = font_size as f32 / 5.0;
        let letter_spacing = font_size as f32 * 0.05;

        // Use font metrics for consistent baseline across all characters
        let ascent = scaled_font.ascent();
        let descent = scaled_font.descent();
        let line_height = ascent - descent;

        let mut total_width = 0.0f32;

        for c in text.chars() {
            let glyph = scaled_font.scaled_glyph(c);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                total_width += bounds.width() + letter_spacing;
            } else {
                let glyph_id = scaled_font.scaled_glyph(c).id;
                total_width += scaled_font.h_advance(glyph_id);
            }
        }

        let img_width = (total_width + margin * 2.0).ceil() as u32;
        let img_height = (line_height + margin * 2.0).ceil() as u32;

        let mut img = GrayImage::new(img_width, img_height);

        for pixel in img.pixels_mut() {
            *pixel = Luma([255]);
        }

        let mut x_offset = margin;
        let baseline_y = margin + ascent;
        for c in text.chars() {
            let glyph = scaled_font.scaled_glyph(c);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let offset_x = (x_offset).floor() as i32;
                let offset_y = (baseline_y + bounds.min.y).floor() as i32;

                let mut coverage_grid = vec![vec![0u8; img_width as usize]; img_height as usize];

                outlined.draw(|x: u32, y: u32, intensity: f32| {
                    let px = x as i32 + offset_x;
                    let py = y as i32 + offset_y;
                    if px >= 0 && py >= 0 {
                        let px = px as u32;
                        let py = py as u32;
                        if px < img_width && py < img_height {
                            let coverage = (intensity * 255.0).round() as u8;
                            if coverage > coverage_grid[py as usize][px as usize] {
                                coverage_grid[py as usize][px as usize] = coverage;
                            }
                        }
                    }
                });

                for y in 0..img_height {
                    for x in 0..img_width {
                        let coverage = coverage_grid[y as usize][x as usize];
                        if coverage > 0 {
                            let pixel = img.get_pixel_mut(x, y);
                            let current_luma = pixel[0];
                            let new_luma = current_luma.saturating_sub(coverage);
                            *pixel = Luma([new_luma]);
                        }
                    }
                }

                x_offset += bounds.width() + letter_spacing;
            } else {
                // Handle space and other characters without outlines
                let glyph_id = scaled_font.scaled_glyph(c).id;
                x_offset += scaled_font.h_advance(glyph_id);
            }
        }

        let mut dots = Vec::new();
        for y in 0..img_height {
            let mut row = Vec::new();
            for x in 0..img_width {
                let pixel = img.get_pixel(x, y);
                let is_dot = pixel[0] < threshold;
                row.push(is_dot);
            }
            dots.push(row);
        }

        Ok(dots)
    }

    fn print_dots(dots: &[Vec<bool>]) {
        let height = dots.len();
        let width = if height > 0 { dots[0].len() } else { 0 };

        let block_height = 3;
        let block_width = 2;

        for y in (0..height).step_by(block_height) {
            let mut line = String::new();
            for x in (0..width).step_by(block_width) {
                if y + block_height > height || x + block_width > width {
                    line.push('\u{2800}');
                    continue;
                }

                let mut pattern = 0u32;
                if dots[y][x] {
                    pattern |= 1 << 0;
                }
                if dots[y + 1][x] {
                    pattern |= 1 << 1;
                }
                if dots[y + 2][x] {
                    pattern |= 1 << 2;
                }
                if dots[y][x + 1] {
                    pattern |= 1 << 3;
                }
                if dots[y + 1][x + 1] {
                    pattern |= 1 << 4;
                }
                if dots[y + 2][x + 1] {
                    pattern |= 1 << 5;
                }

                line.push(char::from_u32(0x2800 + pattern).unwrap());
            }
            println!("{}", line);
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let font_path = cli.font.as_deref().unwrap_or(DEFAULT_FONT);

    if !cli.no_animate {
        let chars: Vec<char> = cli.text.chars().collect();
        let total_chars = chars.len();

        for i in 0..total_chars {
            let text = chars[i].to_string();

            let dots =
                FontToDots::text_to_dots(font_path, &text, cli.font_size, cli.threshold)?;

            clear_screen();
            FontToDots::print_dots(&dots);

            thread::sleep(Duration::from_millis(cli.delay));
        }

        let full_text: String = chars.iter().collect();
        let dots =
            FontToDots::text_to_dots(font_path, &full_text, cli.font_size, cli.threshold)?;

        clear_screen();
        FontToDots::print_dots(&dots);
    } else {
        let dots =
            FontToDots::text_to_dots(font_path, &cli.text, cli.font_size, cli.threshold)?;
        FontToDots::print_dots(&dots);
    }

    Ok(())
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
