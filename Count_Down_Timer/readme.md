# Countdown Timer

A simple Rust countdown timer application that displays a countdown and then pauses execution for a specified duration.

## Description

This program creates a visual countdown from 10 to 1 seconds, then enters a sleep mode for 5 seconds. It's a great demonstration of Rust's concurrency features, specifically thread management and duration handling.

## Features

- ⏱️ **Visual Countdown**: Displays descending seconds from 10 to 1
- 💤 **Thread Sleep**: Pauses execution for a specified duration
- 🚀 **Lightweight**: Efficient thread and time management
- 💻 **Simple Interface**: Clear command-line output
- 📦 **No Dependencies**: Uses only Rust's standard library

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.50.0 or higher)
- Cargo (comes with Rust)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd countdown-timer

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
The timer starts
================
you have 10 seconds left to sleep
you have 9 seconds left to sleep
you have 8 seconds left to sleep
you have 7 seconds left to sleep
you have 6 seconds left to sleep
you have 5 seconds left to sleep
you have 4 seconds left to sleep
you have 3 seconds left to sleep
you have 2 seconds left to sleep
you have 1 seconds left to sleep
entering sleep mode
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/countdown-timer
```

## Project Structure

```
src/
├── main.rs     # Main application logic
```

The application uses only Rust's standard library with no external dependencies.

## Code Overview

```rust
use std::thread;
use std::time::Duration;

fn main() {
    println!("The timer starts");
    println!("================");
    
    let seconds = Duration::from_secs(5);
    let mut val = 10;
    
    loop {
        if val == 0 {
            break;
        }
        println!("you have {} seconds left to sleep", val);
        val -= 1;
    }
    
    println!("entering sleep mode");
    thread::sleep(seconds);
}
```

### How It Works

1. **Initialization**: Sets up a 5-second sleep duration and starts countdown from 10
2. **Countdown Loop**: 
   - Displays the current countdown value
   - Decrements the counter each iteration
   - Breaks when counter reaches 0
3. **Sleep Phase**: Pauses program execution for 5 seconds using `thread::sleep()`

## Building

### Development Build
```bash
cargo build
```

### Release Build (Optimized)
```bash
cargo build --release
```

## Timing Details

- **Countdown Duration**: Approximately 10 seconds (1 second per count)
- **Sleep Duration**: 5 seconds (configurable via `Duration::from_secs(5)`)
- **Total Runtime**: Approximately 15 seconds

## Customization

You can easily modify the timing:

```rust
// Change countdown length
let mut val = 15;  // Count from 15 instead of 10

// Change sleep duration
let seconds = Duration::from_secs(10);  // Sleep for 10 seconds instead of 5
```

## Possible Extensions

- **Configurable Timers**: Command-line arguments for countdown and sleep durations
- **Multiple Intervals**: Series of different timed operations
- **Visual Progress Bars**: Graphical representation of time remaining
- **Sound Alerts**: Audio notifications when timer completes
- **Precise Timing**: More accurate second-by-second countdown
- **Pause/Resume**: Interactive timer controls
- **Multiple Timers**: Manage several countdowns simultaneously

## Use Cases

- **Educational Tool**: Learning about Rust's thread and time modules
- **Productivity Timer**: Pomodoro technique or focus sessions
- **Game Development**: Turn-based game timers
- **Testing**: Simulating delays in application testing
- **Demonstrations**: Showing concurrent operations in Rust
- **Workflow Automation**: Timed sequences in scripts

## Technical Notes

- The countdown displays messages but doesn't actually wait 1 second between messages
- For a real-time countdown, you would add `thread::sleep(Duration::from_secs(1));` inside the loop
- The `thread::sleep()` function blocks the main thread entirely during the sleep phase

## Perfect For

- Rust beginners learning about time and thread operations
- Understanding blocking vs non-blocking operations
- Simple timer implementations
- Demonstrating basic concurrency concepts
- Building blocks for more complex timing applications

Time your operations with this simple yet effective Rust timer! ⏰