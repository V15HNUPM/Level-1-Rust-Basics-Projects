use std::io;

fn main() {
    let mut fib: Vec<u32> = vec![0, 1];
    let mut input = String::new();

    println!("Enter the number of Fibonacci numbers you want to generate: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Not a valid integer");

    match input {
        0 => println!("Please enter a number greater than zero"),
        1 => println!("Fibonacci sequence: {:?}", &fib[0..1]),
        2 => println!("Fibonacci sequence: {:?}", fib),
        _ => {
            for i in 2..input as usize {
                let next = fib[i - 1] + fib[i - 2];
                fib.push(next);
            }
            println!("Fibonacci sequence: {:?}", fib);
        }
    }
}
