/*
This is a simple Rust program that sums the values in a specified column of a CSV file.
It demonstrates how to read command-line arguments, open and read files line by line, split CSV fields, and perform numeric calculations.

Example CSV file (data.csv):
1,2,3
4,5,6
7,8,9

Example usage:
    .\csv-sum.exe data.csv 1
    # Output:
    # 15

The second argument (1) means "sum the values in column 1" (zero-based index).
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name, args[1] is the filename, args[2] is the column index.
    let args: Vec<String> = env::args().collect();

    // Check if the user provided exactly two arguments (filename and column index).
    if args.len() != 3 {
        eprintln!("Usage: csv-sum <filename> <column_index>");
        std::process::exit(1);
    }

    // Open the file specified by the user.
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

    // Parse the column index provided by the user.
    let col: usize = args[2].parse().expect("Invalid column index");

    // Initialize the sum variable to accumulate the values.
    let mut sum = 0.0;

    // Read the file line by line.
    for line in reader.lines() {
        let line = line?;

        // Split the line into fields using a comma as the delimiter.
        let fields: Vec<&str> = line.split(',').collect();

        // Get the value in the specified column, if it exists.
        if let Some(val) = fields.get(col) {
            // Try to parse the value as a floating-point number and add it to the sum.
            if let Ok(num) = val.trim().parse::<f64>() {
                sum += num;
            }
        }
    }

    // Print the final sum to the standard output.
    println!("{}", sum);
    Ok(())
}