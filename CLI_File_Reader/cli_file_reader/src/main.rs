use clap::Parser;
use std::path::Path;
use std::fs;



#[derive(Parser, Debug)]
#[command(name="file-reader",version="0.1.0")]
struct Args{
    #[arg(short,long)]
    file:String,
}


fn main() {
    let args = Args::parse();
    let path=Path::new(&args.file);
    if path.is_file(){
        match fs::read_to_string(path){
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("there has been an issue.{}", e)
        }
    }
    else {
        eprintln!("Error: '{}' is not a valid file or does not exist.", args.file);
    }
}