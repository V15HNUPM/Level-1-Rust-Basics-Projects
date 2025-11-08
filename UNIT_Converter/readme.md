# Unit Converter

A comprehensive command-line unit conversion tool built with Rust that supports length, mass, and temperature conversions across multiple units.

## Description

Unit Converter is a versatile Rust application that provides accurate conversions between different measurement units. It features an interactive command-line interface with robust error handling and supports multiple conversion categories including length, mass, and temperature with various unit combinations.

## Features

- 📏 **Length Conversions**: Meters ↔ Kilometers
- ⚖️ **Mass Conversions**: Grams ↔ Kilograms  
- 🌡️ **Temperature Conversions**: Celsius ↔ Fahrenheit ↔ Kelvin
- 🔄 **Interactive Interface**: Continuous conversion sessions
- 🛡️ **Robust Error Handling**: Comprehensive input validation
- 💻 **User-Friendly**: Clear prompts and formatted output
- 🚀 **High Precision**: Accurate calculations with 4 decimal places
- 📊 **Modular Design**: Organized code structure with separate conversion modules

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd unit-converter

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

### Interactive Session Example

```
Welcome to Unit Converter
=========================

Available conversions:
Length: m, km
Mass: g, kg
Temperature: c, f, k
Type 'quit' to exit

Enter the value to convert: 100
Enter unit to convert from: m
Enter unit to convert to: km
100 m = 0.1000 km

Available conversions:
Length: m, km
Mass: g, kg
Temperature: c, f, k
Type 'quit' to exit

Enter the value to convert: 32
Enter unit to convert from: c
Enter unit to convert to: f
32 c = 89.6000 f
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/unit-converter
```

## Supported Conversions

### Length
- **Meters to Kilometers**: `m → km`
- **Kilometers to Meters**: `km → m`

### Mass
- **Grams to Kilograms**: `g → kg`
- **Kilograms to Grams**: `kg → g`

### Temperature
- **Celsius to Fahrenheit**: `c → f`
- **Celsius to Kelvin**: `c → k`
- **Fahrenheit to Celsius**: `f → c`
- **Fahrenheit to Kelvin**: `f → k`
- **Kelvin to Celsius**: `k → c`
- **Kelvin to Fahrenheit**: `k → f`

## Project Structure

```
src/
├── main.rs          # Main application logic and UI
├── converters/      # Conversion modules
│   ├── mod.rs      # Module declarations
│   ├── length.rs   # Length conversion functions
│   ├── mass.rs     # Mass conversion functions
│   └── temperature.rs # Temperature conversion functions
Cargo.toml          # Project dependencies
```

## Conversion Formulas

### Length
- Meters to Kilometers: `km = m / 1000`
- Kilometers to Meters: `m = km * 1000`

### Mass
- Grams to Kilograms: `kg = g / 1000`
- Kilograms to Grams: `g = kg * 1000`

### Temperature
- Celsius to Fahrenheit: `F = (C × 9/5) + 32`
- Celsius to Kelvin: `K = C + 273.15`
- Fahrenheit to Celsius: `C = (F - 32) × 5/9`
- Fahrenheit to Kelvin: `K = (F - 32) × 5/9 + 273.15`
- Kelvin to Celsius: `C = K - 273.15`
- Kelvin to Fahrenheit: `F = (K - 273.15) × 9/5 + 32`

## Building

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

## Dependencies

This project uses only Rust's standard library - no external dependencies required!

## Error Handling

The application provides comprehensive error handling for:

- **Invalid numeric input**: "Please enter a valid number!"
- **Unsupported unit conversions**: Clear error messages with supported options
- **I/O errors**: Standard input reading failures
- **Graceful exit**: Type 'quit' at any prompt to exit

## Usage Examples

### Common Conversions

**Length:**
```bash
1000 m = 1.0000 km
2.5 km = 2500.0000 m
```

**Mass:**
```bash
500 g = 0.5000 kg
1.75 kg = 1750.0000 g
```

**Temperature:**
```bash
0 c = 32.0000 f
100 c = 212.0000 f
32 f = 0.0000 c
212 f = 100.0000 c
0 c = 273.1500 k
```

## Module Architecture

The application uses a modular design:

- **`main.rs`**: Handles user interaction and program flow
- **`converters/length.rs`**: Length conversion implementations
- **`converters/mass.rs`**: Mass conversion implementations  
- **`converters/temperature.rs`**: Temperature conversion implementations
- **`converters/mod.rs`**: Module exports and organization

## Possible Extensions

- Add more units (miles, feet, pounds, ounces, etc.)
- Support for area and volume conversions
- Currency conversion with live API data
- Command-line arguments for single conversions
- Configuration file for preferred units
- History of recent conversions
- Graphical user interface version

## Perfect For

- **Educational Use**: Learning measurement systems and conversion formulas
- **Scientific Work**: Quick unit conversions in research
- **Engineering**: Technical calculations and unit management
- **Cooking**: Recipe measurement conversions
- **Travel**: International unit conversions
- **Programming Education**: Learning Rust module system and error handling

Enjoy seamless unit conversions with this powerful Rust tool! 🛠️