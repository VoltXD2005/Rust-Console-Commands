// This is a simple Rust program that removes all spaces from a string or file.
// It demonstrates how to read command-line arguments, open files, and process text.

use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // If a single argument is given, remove spaces and print.
    if args.len() == 2 {
        println!("{}", args[1].replace(" ", ""));
    }
    // If "-f" and a filename are given, read the file, remove spaces, and print.
    else if args.len() == 3 && args[1] == "-f" {
        let file = File::open(&args[2])?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        println!("{}", content.replace(" ", ""));
    } else {
        eprintln!("Usage: strip-spaces <string> OR strip-spaces -f <filename>");
    }
    Ok(())
}
