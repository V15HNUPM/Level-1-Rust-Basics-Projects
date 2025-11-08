# BMI Calculator

A simple and efficient command-line BMI (Body Mass Index) calculator written in Rust.

## Description

This Rust program calculates your Body Mass Index (BMI) based on your weight and height inputs, then categorizes the result into standard BMI categories. It's a lightweight, fast, and reliable tool for quick health assessments.

## Features

- 🚀 **Fast and Efficient**: Built with Rust for optimal performance
- 📊 **BMI Calculation**: Accurate BMI computation using standard formula
- 🏥 **Health Categories**: Automatic classification into:
  - Underweight (< 18.5)
  - Healthy (18.5 - 24.9)
  - Overweight (25.0 - 29.9)
  - Obese (≥ 30.0)
- 🛡️ **Safe Input Handling**: Proper error handling for user inputs
- 💻 **Cross-Platform**: Works on any system with Rust installed

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or higher)

### Building from Source

```bash
# Clone the repository (if available)
git clone <repository-url>
cd bmi-calculator

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

2. Follow the prompts:
```
welcome to bmi calculator
=========================
enter your weight : 
70
enter your height : 
175
```

3. View your results:
```
you are healthy
Your BMI is : 22.857143
```

## How BMI is Calculated

The BMI is calculated using the standard formula:

```
BMI = weight (kg) / (height (m))²
```

In code:
```rust
fn bmi(&self) -> f32 {
    self.weight / ((self.height / 100.0) * (self.height / 100.0))
}
```

**Note:** Height is converted from centimeters to meters in the calculation.

## BMI Categories

| Category | BMI Range |
|----------|-----------|
| Underweight | < 18.5 |
| Healthy | 18.5 - 24.9 |
| Overweight | 25.0 - 29.9 |
| Obese | ≥ 30.0 |

## Project Structure

```
src/
├── main.rs     # Main application logic
```

The code uses a `Fields` struct to organize weight and height data, with an associated `bmi()` method to perform the calculation.

## Dependencies

This project uses only Rust's standard library - no external dependencies required!

