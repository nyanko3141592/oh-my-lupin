# oh-my-lupin

Convert text to braille art using any font.

```
⠀⠀⠀⠴⠆⠀⠀⠀⠀⠀⠀⠴⠆⠀⠀⠀⠀⠀⠴⠆⠀⠰⠶⠶⠶⠶⠤⠤⠀⠀⠰⠦⠀⠴⠦⠀⠀⠀⠀⠀⠰⠆⠀⠀⠀⠀
⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠿⠇⠀⠸⠏⠉⠉⠉⠉⠻⠷⠀⠸⠿⠀⠿⠿⠷⠀⠀⠀⠀⠸⠇⠀⠀⠀⠀
⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠿⠇⠀⠸⠇⠀⠀⠀⠀⠀⠿⠂⠸⠿⠀⠿⠇⠻⠧⠀⠀⠀⠸⠇⠀⠀⠀⠀
⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠿⠇⠀⠸⠧⠤⠤⠤⠤⠾⠟⠀⠸⠿⠀⠿⠇⠀⠻⠧⠀⠀⠸⠇⠀⠀⠀⠀
⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠀⠻⠇⠀⠀⠀⠀⠀⠿⠇⠀⠸⠟⠛⠛⠛⠛⠋⠁⠀⠸⠿⠀⠿⠇⠀⠈⠻⠧⠀⠸⠇⠀⠀⠀⠀
⠀⠀⠀⠿⠇⠀⠀⠀⠀⠀⠀⠸⠿⠀⠀⠀⠀⠀⠿⠇⠀⠸⠇⠀⠀⠀⠀⠀⠀⠀⠸⠿⠀⠿⠇⠀⠀⠈⠻⠦⠸⠇⠀⠀⠀⠀
⠀⠀⠀⠿⠧⠤⠤⠤⠤⠤⠀⠈⠿⠦⠀⠀⠠⠴⠿⠁⠀⠸⠇⠀⠀⠀⠀⠀⠀⠀⠸⠿⠀⠿⠇⠀⠀⠀⠈⠻⠿⠇⠀⠀⠀⠀
⠀⠀⠀⠛⠛⠛⠛⠛⠛⠛⠀⠀⠈⠛⠛⠿⠛⠛⠁⠀⠀⠘⠃⠀⠀⠀⠀⠀⠀⠀⠘⠛⠀⠛⠃⠀⠀⠀⠀⠈⠛⠃⠀⠀⠀⠀
```

## Installation

```bash
cargo install oh-my-lupin
```

## Usage

```bash
# Basic usage (uses system default font)
oh-my-lupin "Hello"

# Specify font size
oh-my-lupin "Hello" 80

# Use custom font
oh-my-lupin "Hello" -f /path/to/font.ttf

# Disable animation
oh-my-lupin "Hello" --no-animate

# Adjust threshold (0-255, lower = thinner lines)
oh-my-lupin "Hello" -t 100
```

## Options

| Option | Description |
|--------|-------------|
| `<TEXT>` | Text to display |
| `[FONT_SIZE]` | Font size in pixels (default: 50) |
| `-f, --font <PATH>` | Path to font file (TTF/OTF) |
| `-t, --threshold <N>` | Threshold for dot rendering (default: 128) |
| `-d, --delay <MS>` | Animation delay in milliseconds (default: 200) |
| `--no-animate` | Disable animation |

## Default Fonts

- macOS: Helvetica
- Linux: DejaVuSans
- Windows: Arial

## License

MIT
