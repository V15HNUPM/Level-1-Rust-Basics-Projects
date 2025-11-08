# Colored Hello World

A simple Rust application that demonstrates terminal text coloring and styling using the `colored` crate.

## Description

This project showcases how to add colors and text formatting to terminal output in Rust. It's a minimal example that displays "Hello, world!" in blue italic text, serving as a starting point for building more complex CLI applications with styled output.

## Features

- 🎨 **Colored Text**: Output text in different colors
- ✨ **Text Formatting**: Apply various text styles (italic, bold, underline, etc.)
- 🚀 **Lightweight**: Minimal and efficient
- 💻 **Cross-Platform**: Works on most terminal emulators

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd colored-hello-world

# Build the project
cargo build --release

# Run the application
cargo run
```

## Usage

### Running the Application

```bash
cargo run
```

**Output:**
```
Hello, world!
```
*(Displayed in blue italic text)*

### Direct Execution

```bash
# Run the compiled binary
./target/release/colored-hello-world
```

## Project Structure

```
src/
├── main.rs     # Main application with colored output
Cargo.toml      # Project dependencies
```

## Dependencies

- [`colored`](https://crates.io/crates/colored) = "2.0" - Terminal text coloring and styling

## Code Example

The main functionality is simple but demonstrates the power of the `colored` crate:

```rust
use colored::Colorize;

fn main() {
    println!("{}", "Hello, world!".italic().blue());
}
```

## Extending the Example

You can easily modify the code to experiment with different colors and styles:

```rust
use colored::Colorize;

fn main() {
    println!("{}", "Red bold text!".red().bold());
    println!("{}", "Green underlined!".green().underline());
    println!("{}", "Yellow on blue!".yellow().on_blue());
}
```

## Building

### Development Build

```bash
cargo build
```

### Release Build (Optimized)

```bash
cargo build --release
```

## Supported Terminals

The `colored` crate works on most modern terminals including:
- Linux/Unix terminals
- Windows Command Prompt (Windows 10+)
- Windows Terminal
- macOS Terminal
- iTerm2
- GNOME Terminal
- and more...

## Note

Some older terminals or certain configurations might not support all color and style features. The crate automatically detects terminal capabilities and falls back gracefully when colors are not supported.