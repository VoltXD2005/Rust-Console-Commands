use std::env;
use chrono::NaiveDate;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: date-diff <YYYY-MM-DD> <YYYY-MM-DD>");
        std::process::exit(1);
    }
    let d1 = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Invalid date");
    let d2 = NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").expect("Invalid date");
    let diff = (d2 - d1).num_days().abs();
    println!("{}", diff);
}
// Add to Cargo.toml: chrono = "0.4"
