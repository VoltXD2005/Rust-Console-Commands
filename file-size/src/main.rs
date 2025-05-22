use std::env;
use std::fs::metadata;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: file-size <filename>");
        std::process::exit(1);
    }
    match metadata(&args[1]) {
        Ok(meta) => println!("{}", meta.len()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
