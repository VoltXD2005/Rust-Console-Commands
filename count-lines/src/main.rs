use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: count-lines <filename>");
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let count = reader.lines().count();
    println!("{}", count);
    Ok(())
}
