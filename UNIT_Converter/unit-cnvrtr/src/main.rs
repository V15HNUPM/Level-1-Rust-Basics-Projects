use std::io;
use converters::{length, mass, temperature};

mod converters;

fn main() {
    println!("Welcome to Unit Converter");
    println!("=========================");
    
    loop {
        println!("\nAvailable conversions:");
        println!("Length: m, km");
        println!("Mass: g, kg"); 
        println!("Temperature: c, f, k");
        println!("Type 'quit' to exit");
        
        // Get value to convert
        let value = match get_number_input("Enter the value to convert: ") {
            Ok(val) => val,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Get source unit
        let unit_from = get_unit_input("Enter unit to convert from: ")
            .to_lowercase();

        if unit_from == "quit" {
            break;
        }

        // Get target unit  
        let unit_to = get_unit_input("Enter unit to convert to: ")
            .to_lowercase();

        if unit_to == "quit" {
            break;
        }

        // Perform conversion
        match convert_units(value, &unit_from, &unit_to) {
            Some(result) => {
                println!("{} {} = {:.4} {}", value, unit_from, result, unit_to);
            }
            None => {
                println!("Invalid conversion from {} to {}", unit_from, unit_to);
                println!("Supported conversions:");
                println!("  Length: m ↔ km");
                println!("  Mass: g ↔ kg");
                println!("  Temperature: c ↔ f ↔ k");
            }
        }
    }
    
    println!("Thank you for using Unit Converter!");
}

fn get_number_input(prompt: &str) -> Result<f32, String> {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| "Failed to read input".to_string())?;
    
    let input = input.trim();
    if input.eq_ignore_ascii_case("quit") {
        return Err("quit".to_string());
    }
    
    input.parse::<f32>().map_err(|_| "Invalid number".to_string())
}

fn get_unit_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Flush failed!");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    input.trim().to_string()
}

fn convert_units(value: f32, from: &str, to: &str) -> Option<f32> {
    match (from, to) {
        // Length conversions
        ("m", "km") => Some(length::m_2_km(value)),
        ("km", "m") => Some(length::km_2_m(value)),
        
        // Mass conversions
        ("g", "kg") => Some(mass::g_2_kg(value)),
        ("kg", "g") => Some(mass::kg_2_g(value)),
        
        // Temperature conversions
        ("c", "k") => Some(temperature::c_2_k(value)),
        ("c", "f") => Some(temperature::c_2_f(value)),
        ("k", "c") => Some(temperature::k_2_c(value)),
        ("k", "f") => Some(temperature::k_2_f(value)),
        ("f", "c") => Some(temperature::f_2_c(value)),
        ("f", "k") => Some(temperature::f_2_k(value)),
        
        // Same unit (no conversion needed)
        (a, b) if a == b => Some(value),
        
        // Invalid conversion
        _ => None,
    }
}