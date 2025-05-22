// This is a simple Rust program that counts the number of characters in a string or a file.
// It demonstrates how to read command-line arguments, open files, and perform basic text processing.

// Import modules from the Rust standard library.
// 'env' is used to access command-line arguments.
// 'fs::File' is used to open files.
// 'io' is used for input/output operations and error handling.
// 'BufReader' allows efficient reading from files.
// 'Read' is a trait that enables reading the entire file into a string.
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is always the program name. args[1] is the first argument after the program name.
    let args: Vec<String> = env::args().collect();

    // If the user provides a single argument, count and print the number of characters in it.
    if args.len() == 2 {
        println!("{}", args[1].chars().count());
    }
    // If the user provides "-f" and a filename, open the file and count the characters in its contents.
    else if args.len() == 3 && args[1] == "-f" {
        // Open the file for reading. The '?' operator returns an error if the file can't be opened.
        let file = File::open(&args[2])?;
        // Wrap the file in a buffered reader for efficient reading.
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        // Read the entire file into the 'content' string.
        reader.read_to_string(&mut content)?;
        // Count and print the number of characters in the file.
        println!("{}", content.chars().count());
    }
    // If the arguments are incorrect, print usage instructions.
    else {
        eprintln!("Usage: count-chars <string> OR count-chars -f <filename>");
    }
    // Return Ok(()) to indicate the program finished successfully.
    Ok(())
}
