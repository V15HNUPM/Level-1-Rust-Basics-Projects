# Text Encryptor

Encrypt text using AES encryption in Rust.

## Description
Securely encrypt text with AES-256 encryption using a magic key, outputting results in both Base64 and byte formats.

## Features
- AES-256 encryption
- Base64 and byte array output
- Simple text encryption

## Usage

```bash
cargo run
```

**Example:**
```
Enter text you want to Encrypt: hello world
encrypted of text hello world in base64 is "k3V6K8Z8Sj2R7XlT9wMqNQ=="
encrypted of text hello world in bytes is [123, 45, 234, 167, 89, 123, 234, 211]
```

## Dependencies
- magic-crypt = "3.1"

## Build
```bash
cargo build --release
```
