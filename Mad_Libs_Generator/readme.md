# Mad Libs Game

A fun Rust command-line application that creates hilarious stories by filling in user-provided words into predefined templates.

## Description

This interactive Mad Libs game prompts users for different types of words (nouns, adjectives, places, etc.) and inserts them into a story template to create amusing and often nonsensical results. It's a classic word game brought to life with Rust!

## Features

- 🎮 **Interactive Gameplay**: Prompts users for various word types
- 📝 **Story Templates**: Uses predefined Mad Libs templates
- ✂️ **Input Sanitization**: Trims whitespace from user inputs
- 🎉 **Entertaining Results**: Generates funny and unexpected stories
- 🚀 **Fast Execution**: Built with Rust for quick performance
- 💻 **User-Friendly**: Simple command-line interface

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd mad-libs-game

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

### Game Flow

1. The program welcomes you and displays prompts
2. Enter the requested types of words when prompted
3. Watch as your words are inserted into the story template
4. Enjoy your custom Mad Libs story!

**Example Session:**
```
Welcome to the Mad Libs Game
enter name of a place: supermarket
enter an adjective: sparkly
enter name of a animal: giraffe

here is your mad lib :=> Today I went to the supermarket and saw a sparkly giraffe.
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/mad-libs-game
```

## Project Structure

```
src/
├── main.rs     # Main game logic and user interaction
```

The application uses Rust's standard library for I/O operations and string manipulation.

## How It Works

1. **Template Definition**: A predefined story template with placeholders (`{place}`, `{adjective}`, `{animal}`)
2. **User Input**: Collects words from the user through standard input
3. **String Replacement**: Replaces placeholders with user-provided words
4. **Story Generation**: Outputs the completed Mad Libs story

## Code Overview

```rust
use std::io;

fn main() {
    // Define the Mad Libs template
    let mad_lib = String::from("Today I went to the {place} and saw a {adjective} {animal}.");
    
    // Collect user inputs
    let mut place = String::new();
    let mut adjective = String::new();
    let mut animal = String::new();
    
    // Interactive prompts
    println!("Welcome to the Mad Libs Game");
    println!("enter name of a place: ");
    io::stdin().read_line(&mut place).expect("failed to read");
    
    // ... more prompts
    
    // Replace placeholders with user words
    let mad_lib = mad_lib.replace("{place}", &place.trim());
    let mad_lib = mad_lib.replace("{adjective}", &adjective.trim());
    let mad_lib = mad_lib.replace("{animal}", &animal.trim());
    
    // Display the final story
    println!("here is your mad lib :=> {}", mad_lib);
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

## Example Stories

Here are some examples of what you might create:

**Silly Version:**
```
Today I went to the library and saw a fluffy dinosaur.
```

**Absurd Version:**
```
Today I went to the moon and saw a dancing potato.
```

## Possible Extensions

- Multiple story templates to choose from
- Random template selection
- Colorful output formatting
- Save stories to file
- Share stories with friends
- Add more word types (verbs, adverbs, etc.)
- Create themed Mad Libs (space, fantasy, etc.)

## Perfect For

- Language learning exercises
- Icebreaker activities
- Classroom educational tools
- Family game nights
- Programming beginners learning Rust I/O

Enjoy creating your own wacky stories with this Rust Mad Libs game! 🎭