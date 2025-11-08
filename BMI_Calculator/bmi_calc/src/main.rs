use std::io;

struct Fields {
    weight: f32,
    height: f32,
}

impl Fields {
    fn bmi(&self) -> f32 {
        self.weight / ((self.height / 100.0) * (self.height / 100.0))
    }
}

fn main() {
    let mut w = String::new();
    let mut h = String::new();
    println!("welcome to bmi calculator");
    println!("=========================");
    println!("enter your weight : ");
    io::stdin().read_line(&mut w).expect("failed to read");
    let w: f32 = w.trim().parse().unwrap();
    println!("enter your height : ");
    io::stdin().read_line(&mut h).expect("failed to read");
    let h: f32 = h.trim().parse().unwrap();
    let f = Fields {
        weight: w,
        height: h,
    };
    let bmi_val = f.bmi();
    match bmi_val {
        x if x < 18.5 => println!("you are underweight"),
        x if x > 18.5 && x < 24.9 => println!("you are healthy"),
        x if x > 25.0 && x < 29.9 => println!("you are overweight"),
        x if x > 30.0 => println!("you are obese"),
        _ => println!("invalid"),
    }
    println!("Your BMI is : {}", bmi_val);
}
