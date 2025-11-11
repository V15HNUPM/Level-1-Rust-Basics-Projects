use std::io;

fn main() {
    let mut input = String::new();
    println!("enter which numbers factorial you want to generate: ");
    io::stdin().read_line(&mut input).expect("failed to read");
    let input: u32 = input.trim().parse().expect("not a valid integer");
    match input {
        0 | 1 => println!("factorial is 1"),
        _ => {
            let mut val = 1;
            for i in 2..=input {
                val *= i;
            }
            println!("factorial is {}", val);
        }
    }
}
