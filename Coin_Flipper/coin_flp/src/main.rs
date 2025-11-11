use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let sides = ["tail", "head"];
    let index_val = rng.random_range(0..2);
    println!("lets flip the coin");
    println!("==================");
    println!("the result is {}", sides[index_val]);
}
