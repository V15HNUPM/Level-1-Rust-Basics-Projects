use reqwest::blocking;
use serde_json::Value;

fn main() {
    let response = blocking::get("https://zenquotes.io/api/random")
        .expect("Request failed")
        .text()
        .expect("Failed to read body");

    let v: Value = serde_json::from_str(&response).expect("Invalid JSON");
    let quote = v[0]["q"].as_str().unwrap_or("");

    let author=v[0]["a"].as_str().unwrap_or("");

    println!("{} by {}", quote,author);
}
