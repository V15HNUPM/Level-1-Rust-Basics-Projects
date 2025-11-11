# Hash Generator

Generate cryptographic hashes in Rust.

## Description
Create SHA-1, SHA-2, or SHA-3 hashes from input text using the rust-crypto library.

## Features
- Multiple hash algorithms (SHA-1, SHA-2, SHA-3)
- Secure cryptographic hashing
- Simple text input

## Usage

```bash
cargo run
```

**Examples:**
```
Enter text you want to Hash: hello
Enter which Hashing algorithm you would like to use [sha1, sha2, sha3]: sha1
Hash value of text 'hello' is aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d
```

```
Enter text you want to Hash: password
Enter which Hashing algorithm you would like to use [sha1, sha2, sha3]: sha2
Hash value of text 'password' is 5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8
```

## Dependencies
- rust-crypto = "0.2"

## Build
```bash
cargo build --release
```
