# Guessing Game

A fun and interactive number guessing game built with Rust where players try to guess a randomly generated number.

## Description

Welcome to the Guessing Game! This is a simple yet engaging command-line game where the computer generates a random number between 1 and 10, and you have to guess what it is. The game provides immediate feedback and continues until you guess the correct number.

## Features

- 🎯 **Random Number Generation**: Uses cryptographically secure random number generation
- 🔄 **Interactive Loop**: Continuous gameplay until correct guess
- 📊 **Input Validation**: Basic parsing and error handling
- 🎮 **User-Friendly**: Clear prompts and instructions
- 🚀 **Fast Execution**: Built with Rust for optimal performance
- 💻 **Cross-Platform**: Works on any system with Rust installed

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd guessing-game

# Build the project
cargo build --release

# Run the application
cargo run
```

## Usage

### Running the Game

```bash
cargo run
```

### Gameplay

1. The game starts and welcomes you
2. A random number between 1-10 is generated
3. You're prompted to enter your guess
4. The game continues until you guess correctly

**Example Session:**
```
Welcome To Guessing Game
enter a number between 1 and 10: 
5
enter a number between 1 and 10: 
3
enter a number between 1 and 10: 
7
you have guessed the correct number
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/guessing-game
```

## Project Structure

```
src/
├── main.rs     # Main game logic
Cargo.toml      # Project dependencies
```

## Dependencies

- [`rand`](https://crates.io/crates/rand) = "0.8" - Random number generation library

## How It Works

### Random Number Generation
```rust
let mut rng = rand::rng();
let num: u32 = rng.random_range(1..=10);
```
Generates a cryptographically secure random number between 1 and 10 (inclusive).

### Game Loop
```rust
loop {
    // Get user input
    let mut data = String::new();
    println!("enter a number between 1 and 10: ");
    io::stdin().read_line(&mut data).expect("failed to read");
    
    // Parse input
    let data: u32 = data.trim().parse().unwrap();
    
    // Check guess
    if data == num {
        println!("you have guessed the correct number");
        break;
    }
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

## Game Rules

- The number is always between 1 and 10 (inclusive)
- You can guess as many times as needed
- The game ends only when you guess correctly
- No hints are provided for incorrect guesses (making it more challenging!)

## Possible Enhancements

This basic version can be extended with features like:

- **Hint System**: Tell players if their guess is too high or too low
- **Guess Counter**: Show how many attempts it took to win
- **Difficulty Levels**: Different number ranges (1-20, 1-50, etc.)
- **Score System**: Track best scores or fastest times
- **Input Validation**: Better error handling for invalid inputs
- **Play Again Option**: Restart without exiting the program

## Error Handling

The current version uses basic error handling with `expect()`. In a production version, you might want to add more robust error handling for:
- Non-numeric inputs
- Numbers outside the 1-10 range
- Input/output errors

## Perfect For

- Learning Rust programming concepts
- Understanding game loops and user input
- Demonstrating random number generation
- Beginner programming projects
- Teaching basic game development concepts

Enjoy testing your luck and intuition with this simple yet addictive guessing game! 🎲