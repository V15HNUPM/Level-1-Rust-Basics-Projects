# Level 1 Rust Basics Projects

A collection of beginner-friendly Rust projects that demonstrate fundamental programming concepts through practical applications.

## 📚 Project Overview

This repository contains 15 simple Rust projects designed to help beginners learn Rust programming through hands-on experience. Each project focuses on different core concepts and showcases Rust's safety, performance, and expressiveness.

## 🚀 Projects Included

### 1. **BMI Calculator** 
   - **Description**: Calculate Body Mass Index with health categorization
   - **Concepts**: Structs, methods, pattern matching, user input
   - **File**: `bmi_calculator/`

### 2. **Simple Calculator**
   - **Description**: Basic arithmetic operations (+, -, *, /)
   - **Concepts**: User input, match statements, error handling
   - **File**: `calculator/`

### 3. **File Reader**
   - **Description**: Read and display file contents from command line
   - **Concepts**: CLI arguments with clap, file I/O, error handling
   - **File**: `file_reader/`

### 4. **Colored Hello World**
   - **Description**: Terminal text coloring and styling demonstration
   - **Concepts**: External crates, text formatting
   - **File**: `colored_hello/`

### 5. **Dice Roller**
   - **Description**: Simulate rolling three six-sided dice
   - **Concepts**: Random number generation, functions
   - **File**: `dice_roller/`

### 6. **Quote Generator**
   - **Description**: Fetch random inspirational quotes from API
   - **Concepts**: HTTP requests, JSON parsing, external crates
   - **File**: `quote_generator/`

### 7. **Mad Libs Game**
   - **Description**: Interactive word game creating funny stories
   - **Concepts**: String manipulation, user interaction
   - **File**: `mad_libs/`

### 8. **Text Formatter**
   - **Description**: CLI tool for text transformations (case, trim, count)
   - **Concepts**: Advanced CLI with clap, string operations
   - **File**: `text_formatter/`

### 9. **Guessing Game**
   - **Description**: Classic number guessing game
   - **Concepts**: Loops, random numbers, user input validation
   - **File**: `guessing_game/`

### 10. **Temperature Converter**
   - **Description**: Convert between Celsius, Fahrenheit, and Kelvin
   - **Concepts**: Enums, match statements, mathematical operations
   - **File**: `temperature_converter/`

### 11. **Comprehensive Unit Converter** 
   - **Description**: Multi-category unit converter (length, mass, temperature)
   - **Concepts**: Modular code structure, multiple conversion types
   - **File**: `unit_converter/`

### 12. **Coin Flip Simulator**
   - **Description**: Simulate coin flips with random heads/tails outcomes
   - **Concepts**: Random number generation, arrays
   - **File**: `coin_flip/`

### 13. **Countdown Timer**
   - **Description**: Visual countdown with thread sleep functionality
   - **Concepts**: Thread management, time duration, loops
   - **File**: `countdown_timer/`

### 14. **String Tool**
   - **Description**: Reverse strings and check for palindromes
   - **Concepts**: String manipulation, pattern matching
   - **File**: `string_tool/`

### 15. **Prime Checker**
   - **Description**: Check if numbers are prime using efficient algorithms
   - **Concepts**: Mathematical operations, loops, conditionals
   - **File**: `prime_checker/`

### 16. **Fibonacci Generator**
   - **Description**: Generate Fibonacci sequences of specified length
   - **Concepts**: Vectors, sequences, iterative algorithms
   - **File**: `fibonacci_generator/`

### 17. **Factorial Calculator**
   - **Description**: Calculate factorials using iterative multiplication
   - **Concepts**: Iterative algorithms, mathematical operations
   - **File**: `factorial_calculator/`

### 18. **Hash Generator**
   - **Description**: Create cryptographic hashes (SHA-1, SHA-2, SHA-3)
   - **Concepts**: Cryptography, external crates, hash functions
   - **File**: `hash_generator/`

### 19. **Text Encryptor**
   - **Description**: Encrypt text using AES-256 encryption
   - **Concepts**: Encryption, Base64 encoding, security
   - **File**: `text_encryptor/`

## 🛠️ Prerequisites

- **Rust Programming Language** (version 1.70.0 or higher)
- **Cargo** (Rust's package manager, comes with Rust)
- **Git** (for cloning the repository)

## 📥 Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Or visit [rust-lang.org](https://www.rust-lang.org/tools/install)

2. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/level-1-rust-basics-projects.git
   cd level-1-rust-basics-projects
   ```

## 🏃‍♂️ Running Projects

Each project is in its own directory. To run any project:

```bash
cd project_directory_name
cargo run
```

**Example**:
```bash
cd bmi_calculator
cargo run
```

## 🏗️ Building Projects

To build any project in release mode (optimized):

```bash
cd project_directory_name
cargo build --release
```

The executable will be available at `target/release/project_name`

## 📖 Learning Path

These projects are organized in a suggested learning order:

1. **Basic I/O**: Welcome App, Colored Hello World
2. **Simple Calculations**: Calculator, BMI Calculator, Temperature Converter
3. **Games & Interaction**: Guessing Game, Mad Libs, Dice Roller, Coin Flip
4. **String Operations**: String Tool, Text Formatter
5. **Mathematical Algorithms**: Prime Checker, Fibonacci, Factorial
6. **File & System Operations**: File Reader, Countdown Timer
7. **Network & Security**: Quote Generator, Hash Generator, Text Encryptor
8. **Advanced Patterns**: Unit Converter (modular design)

## 🎯 Key Rust Concepts Covered

- **Variables and Data Types**
- **Control Flow** (if/else, match, loops)
- **Functions and Methods**
- **Structs and Enums**
- **Error Handling**
- **Modules and Crates**
- **Ownership and Borrowing**
- **Pattern Matching**
- **User Input/Output**
- **File Handling**
- **HTTP Requests**
- **CLI Argument Parsing**
- **Random Number Generation**
- **String Manipulation**
- **Mathematical Algorithms**
- **Cryptography and Encryption**
- **Thread and Time Management**

## 🔧 Project Structure

```
level-1-rust-basics-projects/
├── bmi_calculator/
│   ├── src/
│   ├── Cargo.toml
├── calculator/
│   ├── src/
│   ├── Cargo.toml
├── file_reader/
│   ├── src/
│   ├── Cargo.toml
├── colored_hello/
│   ├── src/
│   ├── Cargo.toml
├── dice_roller/
│   ├── src/
│   ├── Cargo.toml
├── quote_generator/
│   ├── src/
│   ├── Cargo.toml
├── mad_libs/
│   ├── src/
│   ├── Cargo.toml
├── text_formatter/
│   ├── src/
│   ├── Cargo.toml
├── guessing_game/
│   ├── src/
│   ├── Cargo.toml
├── temperature_converter/
│   ├── src/
│   ├── Cargo.toml
├── unit_converter/
│   ├── src/
│   ├── Cargo.toml
├── coin_flip/
│   ├── src/
│   ├── Cargo.toml
├── countdown_timer/
│   ├── src/
│   ├── Cargo.toml
├── string_tool/
│   ├── src/
│   ├── Cargo.toml
├── prime_checker/
│   ├── src/
│   ├── Cargo.toml
├── fibonacci_generator/
│   ├── src/
│   ├── Cargo.toml
├── factorial_calculator/
│   ├── src/
│   ├── Cargo.toml
├── hash_generator/
│   ├── src/
│   ├── Cargo.toml
├── text_encryptor/
│   ├── src/
│   ├── Cargo.toml
└── README.md
```

## 🤝 Contributing

Feel free to contribute to this collection by:
1. Adding new beginner-friendly Rust projects
2. Improving existing code with better error handling
3. Adding comments and documentation
4. Creating additional learning resources

## 📝 License

This project collection is open source and available under the [MIT License](LICENSE).

## 🎓 Next Steps

After completing these projects, consider exploring:
- Intermediate Rust concepts (traits, lifetimes, concurrency)
- Web development with Rust (Rocket, Actix)
- Systems programming
- Game development
- CLI tool development
- WebAssembly with Rust

## 📞 Support

If you encounter any issues or have questions:
1. Check the individual project README files
2. Review Rust documentation at [doc.rust-lang.org](https://doc.rust-lang.org/book/)
3. Search Rust community forums and Discord channels

---

**Happy Coding!** 🦀

*Remember: The Rust community is friendly and welcoming to beginners. Don't hesitate to ask questions and share your learning journey!*