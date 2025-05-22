/*
# data.csv contains:
# 1,2,3
# 4,5,6
# 7,8,9

.\csv-sum.exe data.csv 1
# Output:
# 15
*/

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: csv-sum <filename> <column_index>");
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let col: usize = args[2].parse().expect("Invalid column index");
    let mut sum = 0.0;
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if let Some(val) = fields.get(col) {
            if let Ok(num) = val.trim().parse::<f64>() {
                sum += num;
            }
        }
    }
    println!("{}", sum);
    Ok(())
}