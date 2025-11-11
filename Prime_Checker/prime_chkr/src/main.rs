use std::io;

fn main() {
    let mut text = String::new();
    println!("enter a number: ");
    io::stdin().read_line(&mut text).expect("failed to read");
    let num: u32 = text.trim().parse().expect("failed");

    if num <= 1 {
        println!("its not a prime number");
        return;
    }
    if num == 2 {
        println!("its a prime number");
        return;
    }

    let square_root = (num as f64).sqrt() as u32;
    let mut is_prime = true;

    for i in 2..=square_root {
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }

    if is_prime {
        println!("its a prime number");
    } else {
        println!("its not a prime number");
    }
}
