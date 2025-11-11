use std::thread;
use std::time::Duration;
fn main() {
    println!("The timer starts");
    println!("================");
    let seconds = Duration::from_secs(5);
    let mut val = 10;
    loop {
        if val == 0 {
            break;
        }
        println!("you have {} seconds left to sleep", val);
        val -= 1;
    }
    println!("entering sleep mode");
    thread::sleep(seconds);
}
