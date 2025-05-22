/*
This is a simple Rust calculator program that evaluates mathematical expressions from the command line.
It demonstrates how to read command-line arguments, join them into a string, and use an external crate for expression evaluation.

Example usage:
    .\calc.exe 2 + 3 * 4
    # Output:
    # 14

    .\calc.exe "(10 - 2) / 2"
    # Output:
    # 4

Note: This program uses the 'meval' crate to parse and evaluate expressions.
*/

// Import the 'env' module to access command-line arguments.
use std::env;

fn main() {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name. args[1..] are the parts of the expression.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided at least one argument for the expression.
    if args.len() < 2 {
        // Print usage instructions if not enough arguments are provided.
        eprintln!("Usage: calc <expression>");
        std::process::exit(1);
    }

    // Join all arguments after the program name into a single string expression.
    let expr = args[1..].join(" ");

    // Use the 'meval' crate to evaluate the expression string.
    // 'eval_str' returns a Result: Ok(result) if successful, Err(e) if there's an error.
    match meval::eval_str(expr) {
        Ok(result) => println!("{}", result),      // Print the result if successful.
        Err(e) => eprintln!("Error: {}", e),       // Print an error message if evaluation fails.
    }
}

// Add to Cargo.toml dependencies:
// meval = "0.2"