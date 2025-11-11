use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use std::io;

fn main() {
    let mc = new_magic_crypt!("magickey", 256);
    let mut input = String::new();
    println!("Enter text you want to Encrypt: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input = input.trim();
    println!(
        "encrypted of text {}  in bas64 is {:?}",
        input,
        mc.encrypt_str_to_base64(input)
    );
    println!(
        "encrypted of text {}  in bytes is {:?}",
        input,
        mc.encrypt_str_to_bytes(input)
    );
}
