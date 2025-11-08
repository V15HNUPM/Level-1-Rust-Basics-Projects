use clap::{Parser, ValueEnum};


#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    #[value(alias = "u")]
    UPPER,
    #[value(alias = "l")]
    LOWER,
    #[value(alias = "t")]
    TRIM,
    #[value(alias = "c")]
    COUNT,
}

#[derive(Parser, Debug)]
#[command(name="text-formatter",version="0.1.0")]
struct Args{
    #[arg(short,long)]
    text:String,

    mode:Mode,
}


fn main() {
    let args = Args::parse();
    match args.mode{
        Mode::UPPER=>{
            println!("Upper text is {}",args.text.to_uppercase());
        }
        Mode::LOWER=>{
            println!("Lower text is {}",args.text.to_lowercase());
        }
        Mode::TRIM=>{
            let new_text=args.text.replace(" ","");
            println!("Trimmed text is {}",new_text);
        }
        Mode::COUNT=>{
            println!("COUNT text is {}",args.text.chars().count());
        }
    }
}