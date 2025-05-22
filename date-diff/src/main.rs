// This is a simple Rust program that calculates the difference in days between two dates.
// It demonstrates how to read command-line arguments, parse dates, and perform date arithmetic using the 'chrono' crate.

// Import modules from the Rust standard library and the 'chrono' crate.
// 'env' is used to access command-line arguments.
// 'chrono::NaiveDate' is used to represent and parse dates without timezone information.
use std::env;
use chrono::NaiveDate;

fn main() {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name. args[1] and args[2] should be the two dates.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly two arguments (the dates).
    if args.len() != 3 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: date-diff <YYYY-MM-DD> <YYYY-MM-DD>");
        std::process::exit(1);
    }

    // Parse the first date from the first argument.
    // 'parse_from_str' returns a Result; 'expect' will stop the program if parsing fails.
    let d1 = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Invalid date");
    // Parse the second date from the second argument.
    let d2 = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").expect("Invalid date");

    // Calculate the absolute difference in days between the two dates.
    let diff = (d2 - d1).num_days().abs();

    // Print the difference in days.
    println!("{}", diff);
}

// Add to Cargo.toml dependencies:
// chrono = "0.4"
