// This is a simple Rust program that prints the size of a file in bytes.
// It demonstrates how to read command-line arguments and retrieve file metadata using the standard library.

// Import modules from the Rust standard library.
// 'env' is used to access command-line arguments.
// 'fs::metadata' is used to get information about a file, such as its size.
use std::env;
use std::fs::metadata;

fn main() {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name. args[1] should be the filename.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly one argument (the filename).
    if args.len() != 2 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: file-size <filename>");
        std::process::exit(1);
    }

    // Use 'metadata' to get information about the file.
    // If successful, print the file size in bytes.
    // If there is an error (e.g., file not found), print an error message.
    match metadata(&args[1]) {
        Ok(meta) => println!("{}", meta.len()),
        Err(e) => eprintln!("Error: {}", e),
    }
}
