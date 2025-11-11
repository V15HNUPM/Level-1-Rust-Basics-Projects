use std::io;

fn main() {
    let mut text = String::new();
    let mut options = String::new();
    println!("enter text: ");
    io::stdin().read_line(&mut text).expect("failed to read");
    println!("enter options (a or b) : \n a)string reversel\nb)pallindrome checker");
    io::stdin().read_line(&mut options).expect("failed to read");
    let options = options.trim();
    let text = text.trim().to_string();
    match options {
        "a" => {
            let rev: String = text.chars().rev().collect();
            println!("reversed text is {}", rev);
        }
        "b" => {
            let rev: String = text.chars().rev().collect();
            if text == rev {
                println!("the text {} is pallindrome", text);
            } else {
                println!("the text {} is not pallindrome", text);
            }
        }
        _ => println!("invalid option"),
    };
}
