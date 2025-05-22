// This is a simple Rust program that prints a random line from a file.
// It demonstrates how to read command-line arguments, read files, and use random number generation.

// Import necessary modules.
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::SliceRandom; // For selecting a random element.

fn main() -> io::Result<()> {
    // Collect command-line arguments.
    // args[0] is the program name. args[1] should be the filename.
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: random-line <filename>");
        std::process::exit(1);
    }

    // Open the file and read all lines into a vector.
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // Choose and print a random line, if any exist.
    if let Some(line) = lines.choose(&mut rand::thread_rng()) {
        println!("{}", line);
    } else {
        eprintln!("File is empty or could not read lines.");
    }
    Ok(())
}
// Add to Cargo.toml: rand = "0.8"
