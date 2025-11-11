use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
use crypto::sha3::Sha3;
use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter text you want to Hash: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();

    let mut algos = String::new();
    println!("Enter which Hashing algorithm you would like to use [sha1, sha2, sha3]: ");
    io::stdin()
        .read_line(&mut algos)
        .expect("Failed to read input");

    match algos.trim().to_lowercase().as_str() {
        "sha1" => println!("Hash value of text '{}' is {}", input, sha1_algo(input)),
        "sha2" => println!("Hash value of text '{}' is {}", input, sha2_algo(input)),
        "sha3" => println!("Hash value of text '{}' is {}", input, sha3_algo(input)),
        _ => println!("Not a valid option"),
    }
}

fn sha1_algo(hash: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(hash);
    hasher.result_str()
}

fn sha2_algo(hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.input_str(hash);
    hasher.result_str()
}

fn sha3_algo(hash: &str) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(hash);
    hasher.result_str()
}
