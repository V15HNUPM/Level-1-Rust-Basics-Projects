# Rust Calculator

A simple command-line calculator written in Rust that performs basic arithmetic operations.

## Description

This is a lightweight calculator application built in Rust that allows users to perform fundamental mathematical operations through an intuitive command-line interface.

## Features

- ➕ **Addition**: Add two numbers
- ➖ **Subtraction**: Subtract two numbers  
- ✖️ **Multiplication**: Multiply two numbers
- ➗ **Division**: Divide two numbers
- 🚀 **Fast Execution**: Built with Rust for optimal performance
- 💻 **User-Friendly**: Simple interactive prompts
- 🛡️ **Error Handling**: Basic input validation

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or higher)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd rust-calculator

# Build the project
cargo build --release

# Run the application
cargo run
```

## Usage

1. Run the program:
```bash
cargo run
```

2. Follow the interactive prompts:

```
Welcome to my calculator
enter first number: 
10
enter second number: 
5
enter which operation to perform (+,-,*,/) : 
+
```

3. View the result:
```
sum is 15
```

## Supported Operations

- `+` : Addition
- `-` : Subtraction  
- `*` : Multiplication
- `/` : Division (integer division)

## Example

```
Welcome to my calculator
enter first number: 20
enter second number: 4
enter which operation to perform (+,-,*,/) : *
product is 80
```

## Project Structure

The application consists of a single Rust file (`main.rs`) that:
- Reads user input for two numbers
- Accepts an operation choice
- Performs the selected arithmetic operation
- Displays the result

## Technical Details

- Uses Rust's standard library for I/O operations
- Implements basic error handling with `expect()`
- Performs unsigned 32-bit integer arithmetic
- Uses pattern matching for operation selection

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

The executable will be available in `target/debug/` or `target/release/` directory.