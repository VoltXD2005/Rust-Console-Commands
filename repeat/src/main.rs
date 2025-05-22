// This is a simple Rust program that repeats a string a specified number of times.
// It demonstrates how to read command-line arguments and perform basic string operations.

use std::env;

fn main() {
    // args[0] is the program name, args[1] is the string, args[2] is the repeat count.
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: repeat <string> <count>");
        std::process::exit(1);
    }

    let s = &args[1];
    let count: usize = args[2].parse().expect("Count must be a number");

    for _ in 0..count {
        println!("{}", s);
    }
}
