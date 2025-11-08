use std::io;

fn main(){

    println!("Welcome to my calculator");
    let mut num1=String::new();
    println!("enter first number: ");
    io::stdin().read_line(&mut num1).expect("failed to read");
    let num1:u32=num1.trim().parse().unwrap();
    let mut num2=String::new();
    println!("enter second number: ");
    io::stdin().read_line(&mut num2).expect("failed to read");
    let num2:u32=num2.trim().parse().unwrap();
    println!("enter which operation to perform (+,-,*,/) : ");
    let mut op=String::new();
    io::stdin().read_line(&mut op).expect("failed to read");
    let op=op.trim();
    match op{
        "+" => println!("sum is {}",num1+num2),
        "-" => println!("difference is {}",num1-num2),
        "*" => println!("product is {}",num1*num2),
        "/" => println!("dividor is {}",num1/num2),
        _ => println!("invalid operation")
    }
}