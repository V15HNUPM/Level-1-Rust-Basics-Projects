# Dice Roller

A simple Rust application that simulates rolling three six-sided dice.

## Description

This program generates random values for three virtual dice, simulating a classic dice rolling experience. It's a great example of random number generation in Rust and can be used for games, probability testing, or as a learning tool.

## Features

- 🎲 **Three Dice Simulation**: Rolls three virtual six-sided dice
- 🎯 **Random Generation**: Uses Rust's robust random number generation
- 🔢 **Realistic Range**: Generates values between 1-6 (inclusive)
- 🚀 **Fast Execution**: Lightweight and efficient
- 📊 **Clear Output**: Displays all three dice values in a formatted message

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd dice-roller

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

**Example Output:**
```
generated the 3 dice values are 1 => 4, 2 => 1, 3 => 6
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/dice-roller
```

## Project Structure

```
src/
├── main.rs     # Main application logic and dice rolling
Cargo.toml      # Project dependencies and metadata
```

## Dependencies

- [`rand`](https://crates.io/crates/rand) = "0.8" - Random number generation library

## Code Overview

The application consists of two main functions:

### Main Function
```rust
fn main() {
    let dice1 = gen_val();
    let dice2 = gen_val();
    let dice3 = gen_val();
    println!("generated the 3 dice values are 1 => {}, 2 => {}, 3 => {}", dice1, dice2, dice3);
}
```

### Random Value Generator
```rust
fn gen_val() -> u32 {
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=6);
    n
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

### Running Tests
```bash
cargo test
```

## Example Use Cases

- **Board Games**: Replace physical dice in tabletop games
- **Game Development**: As a component in larger game systems
- **Probability Education**: Demonstrate random distribution
- **Decision Making**: Use for random selections

## Possible Extensions

This basic dice roller can be extended with features like:
- Multiple dice types (d4, d8, d10, d12, d20)
- Rolling multiple dice of the same type
- Keeping highest/lowest rolls
- Roll history tracking
- Graphical dice representation

## Note

The random number generation uses Rust's cryptographically secure random number generator, ensuring fair and unpredictable dice rolls.# just simple dice roller which uses rand crate to generate 3 dice rolls