# Random Quote Generator

A Rust command-line application that fetches and displays random inspirational quotes from the ZenQuotes API.

## Description

This application connects to the ZenQuotes API to retrieve random inspirational quotes and display them in your terminal. It's a simple yet powerful demonstration of making HTTP requests and parsing JSON data in Rust.

## Features

- 🌟 **Random Quotes**: Fetches random inspirational quotes
- 🌐 **HTTP Requests**: Uses reqwest for API communication
- 📄 **JSON Parsing**: Efficiently parses API responses with serde_json
- 🚀 **Synchronous**: Simple blocking API calls for straightforward usage
- 💫 **Inspirational**: Great for daily motivation or terminal startup messages

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 or higher)
- Cargo (comes with Rust)
- Internet connection (for API calls)

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd random-quote-generator

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
The only way to do great work is to love what you do. by Steve Jobs
```

### Direct Execution

```bash
# Run the compiled binary
./target/release/random-quote-generator
```

## Project Structure

```
src/
├── main.rs     # Main application logic
Cargo.toml      # Project dependencies
```

## Dependencies

- [`reqwest`](https://crates.io/crates/reqwest) = "0.11" - HTTP client for making API requests
- [`serde_json`](https://crates.io/crates/serde_json) = "1.0" - JSON parsing and serialization

## API Reference

This application uses the [ZenQuotes.io API](https://zenquotes.io/):

- **Endpoint**: `https://zenquotes.io/api/random`
- **Method**: GET
- **Response**: JSON array with quote objects
- **Rate Limit**: Free tier available

### Response Format
```json
[
  {
    "q": "The quote text",
    "a": "Author name",
    "h": "HTML formatted quote"
  }
]
```

## Code Overview

```rust
use reqwest::blocking;
use serde_json::Value;

fn main() {
    // Make HTTP request to ZenQuotes API
    let response = blocking::get("https://zenquotes.io/api/random")
        .expect("Request failed")
        .text()
        .expect("Failed to read body");

    // Parse JSON response
    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    
    // Extract quote and author
    let quote = v[0]["q"].as_str().unwrap_or("");
    let author = v[0]["a"].as_str().unwrap_or("");

    // Display the result
    println!("{} by {}", quote, author);
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

The application includes basic error handling for:
- Network connectivity issues
- API unavailability
- JSON parsing errors
- Missing data fields

## Possible Extensions

- Add command-line arguments for multiple quotes
- Implement caching to reduce API calls
- Add different quote categories
- Format output with colors and styling
- Create a desktop notification version
- Add scheduling for daily quotes

## Note

This application requires an active internet connection to fetch quotes from the ZenQuotes API. The free tier of the API has rate limits, so consider caching if you plan to make frequent requests.