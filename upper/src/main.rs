// This is a simple Rust program that converts a string or file contents to uppercase.
// It demonstrates how to read command-line arguments, open files, and process text.

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // If a single argument is given, print it in uppercase.
    if args.len() == 2 {
        println!("{}", args[1].to_uppercase());
    }
    // If "-f" and a filename are given, read the file and print each line in uppercase.
    else if args.len() == 3 && args[1] == "-f" {
        let file = File::open(&args[2])?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line?.to_uppercase());
        }
    } else {
        eprintln!("Usage: upper <string> OR upper -f <filename>");
    }
    Ok(())
}
