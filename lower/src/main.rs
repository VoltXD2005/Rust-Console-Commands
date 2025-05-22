// This is a simple Rust program that converts a string to lowercase.
// It demonstrates how to read command-line arguments and perform basic string manipulation.

// Import the 'env' module to access command-line arguments.
use std::env;

fn main() {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name. args[1] should be the string to convert.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly one argument (the string to convert).
    if args.len() != 2 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: lower <string>");
        std::process::exit(1);
    }

    // Get a reference to the input string.
    let s = &args[1];
    // Convert the string to lowercase using the 'to_lowercase' method.
    let lower = s.to_lowercase();
    // Print the lowercase string.
    println!("{}", lower);
}
