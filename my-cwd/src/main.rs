// This is a simple Rust program that prints the current working directory.
// It demonstrates how to use the standard library to get and display the current directory.

// Import the 'env' module to access environment-related functions.
use std::env;

fn main() {
    // Attempt to get the current working directory using 'current_dir'.
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()), // Print the directory path if successful.
        Err(e) => {
            // Print an error message if unable to get the directory.
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
