// This is a simple Rust program that checks if a given string can be parsed as a number.
// It demonstrates how to read command-line arguments and perform basic type parsing.

// Import the 'env' module to access command-line arguments.
use std::env;

fn main() {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name. args[1] should be the string to check.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly one argument (the string to check).
    if args.len() != 2 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: is-number <string>");
        std::process::exit(1);
    }

    // Get a reference to the input string.
    let s = &args[1];
    // Try to parse the string as a floating-point number (f64).
    // 'is_ok()' returns true if parsing succeeds, false otherwise.
    let is_num = s.parse::<f64>().is_ok();
    // Print 'true' if the string is a number, 'false' otherwise.
    println!("{}", is_num);
}
