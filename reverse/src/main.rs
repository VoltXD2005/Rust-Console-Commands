/*
.\reverse.exe "hello world"
# Output:
# dlrow olleh

# lines.txt contains:
# hello
# world

.\reverse.exe -f lines.txt
# Output:
# olleh
# dlrow
*/

// This is a simple Rust program that reverses a string.
// It demonstrates how to read command-line arguments and manipulate strings.

use std::env;

fn main() {
    // args[0] is the program name, args[1] is the string to reverse.
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: reverse <string>");
        std::process::exit(1);
    }

    let s = &args[1];
    // Reverse the string using 'chars().rev().collect()'.
    let reversed: String = s.chars().rev().collect();
    println!("{}", reversed);
}