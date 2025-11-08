# Temperature Converter

A command-line temperature conversion tool built with Rust that converts between Celsius and Fahrenheit scales.

## Description

This Rust application provides an intuitive way to convert temperatures between Celsius and Fahrenheit. It features a clean command-line interface, robust input validation, and accurate mathematical conversions using standard temperature conversion formulas.

## Features

- 🌡️ **Bidirectional Conversion**: Convert from Celsius to Fahrenheit and vice versa
- 🎯 **Precise Calculations**: Uses exact mathematical formulas for accurate conversions
- 🛡️ **Input Validation**: Robust error handling for invalid inputs and temperature types
- 💻 **User-Friendly**: Interactive prompts with clear instructions
- 🚀 **Fast Execution**: Efficient Rust implementation for instant conversions
- 📊 **Formatted Output**: Clean, readable results with proper formatting

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd temperature-converter

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

### Example Sessions

**Converting Celsius to Fahrenheit:**
```
Temperature Converter
=====================
Enter temperature value: 
25
Which format should convert to (F/C): 
F
25°C is equal to 77.0°F
```

**Converting Fahrenheit to Celsius:**
```
Temperature Converter
=====================
Enter temperature value: 
98.6
Which format should convert to (F/C): 
C
98.6°F is equal to 37.0°C
```

**Invalid Input Handling:**
```
Temperature Converter
=====================
Enter temperature value: 
abc
Please enter a valid number
```

**Invalid Temperature Type:**
```
Which format should convert to (F/C): 
K
error invalid
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/temperature-converter
```

## Conversion Formulas

The application uses standard temperature conversion formulas:

- **Celsius to Fahrenheit**: `F = (C × 9/5) + 32`
- **Fahrenheit to Celsius**: `C = (F - 32) × 5/9`

## Project Structure

```
src/
├── main.rs     # Main application logic and temperature enum
```

The application uses only Rust's standard library with no external dependencies.

## Code Overview

### Temperature Enum

```rust
enum Temp {
    F,
    C,
    Err(String)
}

impl Temp {
    fn convrt(s: &str) -> Temp {
        let trimmed = s.trim();
        if trimmed == "F" {
            Temp::F
        } else if trimmed == "C" {
            Temp::C
        } else {
            Temp::Err(String::from("error invalid"))
        }
    }
}
```

### Main Conversion Logic

```rust
match Temp::convrt(&temp_type) {
    Temp::F => {
        // Convert C to F
        let val = (temp * 9.0 / 5.0) + 32.0;
        println!("{}°C is equal to {:.1}°F", temp, val)
    },
    Temp::C => {
        // Convert F to C
        let val = (temp - 32.0) * 5.0 / 9.0;
        println!("{}°F is equal to {:.1}°C", temp, val)
    },
    Temp::Err(msg) => println!("{}", msg),
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

## Error Handling

The application provides comprehensive error handling for:

- **Non-numeric temperature values**: "Please enter a valid number"
- **Invalid temperature types**: "error invalid" for anything other than F or C
- **I/O errors**: Standard input reading failures

## Common Temperature References

| Description | Celsius | Fahrenheit |
|-------------|---------|------------|
| Water Freezes | 0°C | 32°F |
| Room Temperature | 20°C | 68°F |
| Human Body | 37°C | 98.6°F |
| Water Boils | 100°C | 212°F |

## Possible Enhancements

- Add Kelvin scale support
- Command-line arguments for batch conversions
- Temperature range warnings (extreme hot/cold)
- Historical temperature data
- Multiple unit conversions in one session
- Graphical user interface version
- Temperature conversion charts

## Perfect For

- **Educational Use**: Learning temperature conversion formulas
- **Weather Applications**: Converting between temperature scales
- **Cooking**: Recipe temperature conversions
- **Scientific Work**: Quick temperature calculations
- **Travel**: Converting between metric and imperial systems

## Accuracy

The application provides precise conversions with one decimal place accuracy, suitable for most practical applications including scientific, culinary, and meteorological use cases.