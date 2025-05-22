use std::env;
use chrono::NaiveDate;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: day-of-week <YYYY-MM-DD>");
        std::process::exit(1);
    }
    let date = NaiveDate::parse_from_str(&args[1], "%Y-%m-%d").expect("Invalid date format");
    println!("{}", date.format("%A"));
}
// Add to Cargo.toml: chrono = "0.4"
