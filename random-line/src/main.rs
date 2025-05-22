use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::IteratorRandom;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: random-line <filename>");
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut rng = rand::thread_rng();
    if let Some(line) = reader.lines().filter_map(Result::ok).choose(&mut rng) {
        println!("{}", line);
    }
    Ok(())
}
// Add to Cargo.toml: rand = "0.8"
