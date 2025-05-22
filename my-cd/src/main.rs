// This is a simple Rust program that attempts to change the current working directory.
// It demonstrates how to read command-line arguments and use standard library functions for directory management.

// Import the 'env' module to access command-line arguments and change directories.
use std::env;

fn main() {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name. args[1] should be the directory to change to.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly one argument (the directory to change to).
    if args.len() != 2 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: my-cd <directory>");
        std::process::exit(1);
    }

    // Get a reference to the directory path.
    let dir = &args[1];

    // Attempt to change the current working directory using 'set_current_dir'.
    // If it fails, print an error message.
    // ****  This program needs to be executed from inside a shell to work
    // ****  properly. It won't change the directory of the parent shell.
    // ****  This is a limitation of how child processes work in operating systems.
    if let Err(e) = env::set_current_dir(dir) {
        eprintln!("Error changing directory: {}", e);
        std::process::exit(1);
    }
    // If successful, print a confirmation message.
    println!("Changed directory to '{}'", dir);
}