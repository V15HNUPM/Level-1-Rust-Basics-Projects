use std::io;

fn main(){
    let mut name=String::new();
    
    println!("lets Get to know each other!");

    println!("please enter your name : ");

    io::stdin().read_line(&mut name).expect("failed to read");

    println!("Hallo {} Welcome to our application",name);
}