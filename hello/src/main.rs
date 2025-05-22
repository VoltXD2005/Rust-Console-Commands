// This is a simple Rust program that prints a greeting to the user.
// It demonstrates how to read command-line arguments and print output.

// Import modules from the Rust standard library.
// 'env' is used to access command-line arguments.
// 'io' is used for input/output operations and error handling.
use std::env;
use std::io;       // Used for error handling in the main function.

// The main function is where every Rust program starts executing.
// The '-> io::Result<()>' means this function can return an input/output error.
// This allows us to use '?' for error handling if we add file or input/output code later.
// e.g. let file = File::open("foo.txt")?;
// The '?' operator will return an error if the file cannot be opened.
// The '()' indicates that the function does not return any value.
fn main() -> io::Result<()> {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is always the program name. args[1] is the first argument after the program name.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly one argument (besides the program name).
    if args.len() == 2 {
        // Print a greeting using the provided argument.
        // '{}' is a placeholder for the argument value.
        println!("Hello, {}!", args[1]);
    } else {
        // If the user did not provide the correct number of arguments, print a usage message.
        // 'eprintln!' prints to the error output stream.
        eprintln!("Usage: hello <string>");
    }
    // Return Ok(()) to indicate the program finished successfully.
    Ok(())
}
