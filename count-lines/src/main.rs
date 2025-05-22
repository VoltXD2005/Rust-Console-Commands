// This is a simple Rust program that counts the number of lines in a file.
// It demonstrates how to read command-line arguments, open files, and process lines efficiently.

// Import modules from the Rust standard library.
// 'env' is used to access command-line arguments.
// 'fs::File' is used to open files.
// 'io' is used for input/output operations and error handling.
// 'BufRead' and 'BufReader' are used for efficient line-by-line reading.
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is always the program name. args[1] should be the filename.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly one argument (the filename).
    if args.len() != 2 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: count-lines <filename>");
        std::process::exit(1);
    }

    // Open the file for reading. The '?' operator returns an error if the file can't be opened.
    let file = File::open(&args[1])?;
    // Wrap the file in a buffered reader for efficient line-by-line reading.
    let reader = BufReader::new(file);
    // Count the number of lines in the file using the 'lines' iterator.
    let count = reader.lines().count();
    // Print the line count.
    println!("{}", count);
    // Return Ok(()) to indicate the program finished successfully.
    Ok(())
}
