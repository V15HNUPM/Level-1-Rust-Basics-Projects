# File Reader

A simple and efficient command-line file reader built with Rust.

## Description

File Reader is a lightweight CLI tool that reads and displays the contents of text files. Built with Rust's performance and safety features, it provides a reliable way to quickly view file contents from the terminal.

## Features

- 📁 **File Reading**: Quickly read and display file contents
- 🚀 **High Performance**: Built with Rust for optimal speed
- 🛡️ **Error Handling**: Comprehensive error messages for various file issues
- 🎯 **CLI Interface**: User-friendly command-line arguments
- 📦 **Zero Dependencies**: Only uses `clap` for argument parsing

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd file-reader

# Build in release mode
cargo build --release

# The binary will be available at target/release/file-reader
```

## Usage

### Basic Usage

```bash
# Read a file
file-reader --file example.txt

# Or using short form
file-reader -f example.txt
```

### Help Information

```bash
# Display help
file-reader --help

# Output:
file-reader 0.1.0
file-reader --file <FILE>

OPTIONS:
    -f, --file <FILE>
    -h, --help          Print help information
    -V, --version       Print version information
```

## Examples

### Reading a Text File

```bash
$ file-reader -f README.md
# Displays the contents of README.md
```

### File Not Found

```bash
$ file-reader -f nonexistent.txt
Error: 'nonexistent.txt' is not a valid file or does not exist.
```

### Permission Issues

```bash
$ file-reader -f /root/protected.txt
there has been an issue: Permission denied (os error 13)
```

## Project Structure

```
src/
├── main.rs          # Main application logic
Cargo.toml          # Project dependencies and metadata
```

## Dependencies

- `clap = "4.0"` - Command line argument parsing

## Building

### Development Build

```bash
cargo build
```

### Release Build (Optimized)

```bash
cargo build --release
```

### Running Directly

```bash
cargo run -- --file example.txt
```

## API

The application uses a simple struct for command-line arguments:

```rust
#[derive(Parser, Debug)]
#[command(name="file-reader",version="0.1.0")]
struct Args {
    #[arg(short,long)]
    file: String,
}
```

## Error Handling

The tool provides clear error messages for:
- File not found
- Permission denied
- Invalid file paths
- Non-file inputs (directories)
- Read errors

