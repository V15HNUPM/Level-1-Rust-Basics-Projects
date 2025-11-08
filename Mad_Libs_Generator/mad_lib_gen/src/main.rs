use std::io;

fn main(){
    let mad_lib=String::from("Today I went to the {place} and saw a {adjective} {animal}.");
    let mut place=String::new();
    let mut adjective=String::new();
    let mut animal=String::new();
    println!("Welcome to the Mad Libs Game");
    println!("enter name of a place: ");
    io::stdin().read_line(&mut place).expect("failed to read");
    println!("enter an adjective: ");
    io::stdin().read_line(&mut adjective).expect("failed to read");
    println!("enter name of a animal: ");
    io::stdin().read_line(&mut animal).expect("failed to read");
    let mad_lib=mad_lib.replace("{place}",&place.trim());
    let mad_lib=mad_lib.replace("{adjective}",&adjective.trim());
    let mad_lib=mad_lib.replace("{animal}",&animal.trim());
    println!("here is your mad lib :=> {}",mad_lib);
}