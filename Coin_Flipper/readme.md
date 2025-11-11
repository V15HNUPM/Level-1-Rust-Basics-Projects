# Coin Flip Simulator

A simple Rust application that simulates flipping a coin with random outcomes of heads or tails.

## Description

This lightweight program simulates the classic coin flip experiment using Rust's random number generation capabilities. It's a perfect example for beginners learning about random number generation and array operations in Rust.

## Features

- 🎲 **Random Coin Flips**: Generates truly random heads or tails outcomes
- ⚡ **Fast Execution**: Quick and efficient random number generation
- 💻 **Simple Interface**: Clean command-line output
- 🎯 **Accurate Probabilities**: 50/50 chance for heads or tails
- 🚀 **Zero Dependencies**: Only uses the `rand` crate for randomness

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd coin-flip-simulator

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

### Example Output

```
lets flip the coin
==================
the result is head
```

Or:

```
lets flip the coin
==================
the result is tail
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/coin-flip-simulator
```

## Project Structure

```
src/
├── main.rs     # Main application logic
Cargo.toml      # Project dependencies
```

## Dependencies

- [`rand`](https://crates.io/crates/rand) = "0.8" - Random number generation library

## Code Overview

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let sides = ["tail", "head"];
    let index_val = rng.random_range(0..2);
    println!("lets flip the coin");
    println!("==================");
    println!("the result is {}", sides[index_val]);
}
```

### How It Works

1. **Random Number Generation**: Creates a random number generator instance
2. **Coin Sides Array**: Defines the two possible outcomes in an array
3. **Random Index**: Generates a random index (0 or 1) to select from the array
4. **Output**: Displays the coin flip result with a formatted message

## Building

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

## Probability

The simulator maintains a true 50/50 probability distribution:
- **Heads**: 50% chance
- **Tails**: 50% chance

This is achieved through Rust's cryptographically secure random number generator.

## Possible Extensions

- Multiple consecutive flips
- Flip counting and statistics
- Best-of series simulations
- Graphical coin animation
- Command-line arguments for number of flips
- History of previous flips
- Bias simulation for educational purposes

## Use Cases

- **Decision Making**: When you can't decide between two options
- **Game Development**: As a random event generator for games
- **Probability Education**: Teaching basic probability concepts
- **Testing**: As a simple random binary generator
- **Learning Tool**: Understanding random number generation in Rust

## Fun Facts

- The original code uses "tail" first in the array, then "head"
- This follows the common convention of listing tails before heads
- The random range `0..2` generates numbers from 0 to 1 (inclusive start, exclusive end)

## Perfect For

- Rust beginners learning about arrays and random numbers
- Quick decision-making tool
- Educational demonstrations of randomness
- Simple game mechanics
- Testing random number generation concepts

Flip the coin and let chance decide! 🪙