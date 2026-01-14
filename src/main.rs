use ab_glyph::{Font, FontRef, ScaleFont};
use anyhow::{anyhow, Result};
use clap::Parser;
use image::{GrayImage, Luma};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "font2dots")]
#[command(about = "Convert text to braille using any font", long_about = None)]
struct Cli {
    #[arg(help = "Path to font file (TTF/OTF)")]
    font_path: String,

    #[arg(help = "Text to display")]
    text: String,

    #[arg(help = "Font size in pixels")]
    font_size: u32,

    #[arg(
        short,
        long,
        default_value = "128",
        help = "Threshold for dot rendering (0-255, default: 128)"
    )]
    threshold: u8,
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

        let mut total_width = 0.0f32;
        let mut max_height: f32 = 0.0;

        for c in text.chars() {
            let glyph = scaled_font.scaled_glyph(c);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                total_width += bounds.width();
                max_height = max_height.max(bounds.height());
            }
        }

        let img_width = (total_width + margin * 2.0).ceil() as u32;
        let img_height = (max_height + margin * 2.0).ceil() as u32;

        let mut img = GrayImage::new(img_width, img_height);

        for pixel in img.pixels_mut() {
            *pixel = Luma([255]);
        }

        let mut x_offset = margin;
        for c in text.chars() {
            let glyph = scaled_font.scaled_glyph(c);
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let offset_x = (x_offset + bounds.min.x).floor() as i32;
                let offset_y = (margin).floor() as i32;

                let mut coverage_grid = vec![vec![0u8; img_width as usize]; img_height as usize];

                outlined.draw(|x, y, intensity| {
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

                x_offset += bounds.width();
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

    let dots = FontToDots::text_to_dots(&cli.font_path, &cli.text, cli.font_size, cli.threshold)?;
    FontToDots::print_dots(&dots);

    Ok(())
}
