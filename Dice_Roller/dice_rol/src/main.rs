use rand::Rng;


fn main(){
    let dice1=gen_val();
    let dice2=gen_val();
    let dice3=gen_val();
    println!("generated the 3 dice values are 1 => {}, 2 => {}, 3 => {}",dice1,dice2,dice3);
}


fn gen_val() -> u32{
    let mut rng=rand::rng();
    let n:u32=rng.random_range(1..=6);
    n
}