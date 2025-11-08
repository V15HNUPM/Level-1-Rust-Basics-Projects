# Text Formatter

A powerful command-line text manipulation tool built with Rust that provides various text transformation operations.

## Description

Text Formatter is a versatile CLI utility that performs multiple text transformations including case conversion, whitespace removal, and character counting. Built with the `clap` library for robust command-line argument parsing, it's perfect for scripting, text processing pipelines, or quick text manipulations.

## Features

- 🔠 **Case Conversion**: Convert text to uppercase or lowercase
- ✂️ **Whitespace Removal**: Remove all spaces from text
- 🔢 **Character Counting**: Count total characters in text
- 🎯 **Intuitive CLI**: Easy-to-use command-line interface with short aliases
- 🚀 **High Performance**: Built with Rust for fast text processing
- 📦 **Zero Runtime Dependencies**: Standalone binary after compilation

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd text-formatter

# Build in release mode
cargo build --release

# The binary will be available at target/release/text-formatter
```

## Usage

### Basic Syntax

```bash
text-formatter --text "Your Text Here" <MODE>
```

### Available Modes

| Mode | Command | Alias | Description |
|------|---------|-------|-------------|
| UPPER | `UPPER` | `u` | Convert text to uppercase |
| LOWER | `LOWER` | `l` | Convert text to lowercase |
| TRIM | `TRIM` | `t` | Remove all whitespace |
| COUNT | `COUNT` | `c` | Count characters |

### Examples

**Uppercase Conversion:**
```bash
text-formatter --text "Hello World" UPPER
# or using alias:
text-formatter --text "Hello World" u
```
**Output:** `Upper text is HELLO WORLD`

**Lowercase Conversion:**
```bash
text-formatter --text "Hello World" LOWER
# or using alias:
text-formatter --text "Hello World" l
```
**Output:** `Lower text is hello world`

**Remove Whitespace:**
```bash
text-formatter --text "Hello World" TRIM
# or using alias:
text-formatter --text "Hello World" t
```
**Output:** `Trimmed text is HelloWorld`

**Character Count:**
```bash
text-formatter --text "Hello World" COUNT
# or using alias:
text-formatter --text "Hello World" c
```
**Output:** `COUNT text is 11`

### Help Information

```bash
text-formatter --help
```

**Output:**
```
text-formatter 0.1.0

USAGE:
    text-formatter [OPTIONS] --text <TEXT> <MODE>

ARGS:
    <MODE> [possible values: upper, u, lower, l, trim, t, count, c]

OPTIONS:
    -t, --text <TEXT>
    -h, --help          Print help information
    -V, --version       Print version information
```

## Project Structure

```
src/
├── main.rs          # Main application logic
Cargo.toml          # Project dependencies and metadata
```

## Dependencies

- [`clap`](https://crates.io/crates/clap) = "4.0" - Command line argument parsing with derive macros

## API Reference

### Command Line Arguments

```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(short,long)]
    text: String,
    mode: Mode,  // UPPER, LOWER, TRIM, or COUNT
}
```

### Transformation Modes

- **UPPER**: Uses `text.to_uppercase()`
- **LOWER**: Uses `text.to_lowercase()`
- **TRIM**: Uses `text.replace(" ", "")`
- **COUNT**: Uses `text.chars().count()`

## Building

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

### Installation to Cargo Bin
```bash
cargo install --path .
```

## Integration Examples

### Shell Scripts
```bash
#!/bin/bash
# Convert filename to lowercase
filename=$(text-formatter --text "MY_FILE.TXT" l)
echo "Processed: $filename"
```

### Pipeline Processing
```bash
echo "Some Text With Spaces" | xargs -I {} text-formatter --text "{}" t
```

### Batch Processing
```bash
for word in "Hello" "World" "Rust Programming"; do
    text-formatter --text "$word" u
done
```

## Use Cases

- **Data Cleaning**: Prepare text for databases or APIs
- **File Renaming**: Standardize filenames in scripts
- **Text Analysis**: Count characters for validation
- **API Development**: Format text payloads
- **Educational Tool**: Learn text processing in Rust

## Performance

The tool efficiently handles:
- Small to medium text strings (optimal performance)
- Unicode characters (proper character counting)
- Large texts (memory efficient processing)

