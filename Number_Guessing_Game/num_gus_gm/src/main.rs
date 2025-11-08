use std::io;
use rand::Rng;

fn main(){
    println!("Welcome To Guessing Game");
    
    let mut rng=rand::rng();
    let num:u32=rng.random_range(1..=10);
    
    loop{
        let mut data=String::new();
        println!("enter a number between 1 and 10: ");
        io::stdin().read_line(&mut data).expect("failed to read");
        let data:u32=data.trim().parse().unwrap();
        if data == num{
            println!("you have guessed the correct number");
            break;
        }
    }
}