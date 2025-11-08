use std::io;

enum Temp {
    F,
    C,
    Err(String)
}

impl Temp {
    fn convrt(s: &str) -> Temp {
        let trimmed = s.trim(); // Trim whitespace and newline characters
        if trimmed == "F" {
            Temp::F
        } else if trimmed == "C" {
            Temp::C
        } else {
            Temp::Err(String::from("error invalid"))
        }
    }
}

fn main() {
    // C => F  [ F = (C * 9/5) + 32]
    // F => C  [ C = (F - 32) * 5/9]

    println!("Temperature Converter");
    println!("=====================");
    
    let mut temp = String::new();
    println!("Enter temperature value: ");
    io::stdin().read_line(&mut temp).expect("Failed to read input");
    let temp: f64 = temp.trim().parse().expect("Please enter a valid number");
    
    println!("Which format should convert to (F/C): ");
    let mut temp_type = String::new();
    io::stdin().read_line(&mut temp_type).expect("Failed to read input");
    
    match Temp::convrt(&temp_type) {
        Temp::F => {
            // Convert C to F
            let val = (temp * 9.0 / 5.0) + 32.0;
            println!("{}°C is equal to {:.1}°F", temp, val)
        },
        Temp::C => {
            // Convert F to C
            let val = (temp - 32.0) * 5.0 / 9.0;
            println!("{}°F is equal to {:.1}°C", temp, val)
        },
        Temp::Err(msg) => println!("{}", msg),
    }
}