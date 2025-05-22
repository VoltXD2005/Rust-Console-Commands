// This is a simple Rust program that determines the zodiac sign for a given date.
// It demonstrates how to read command-line arguments, parse input, and use pattern matching.

use chrono::NaiveDate;
use std::env;

// This function determines the zodiac sign based on month and day.
fn zodiac_sign(month: u32, day: u32) -> &'static str {
    match (month, day) {
        (1, 20..) | (2, ..=18) => "Aquarius",
        (2, 19..) | (3, ..=20) => "Pisces",
        (3, 21..) | (4, ..=19) => "Aries",
        (4, 20..) | (5, ..=20) => "Taurus",
        (5, 21..) | (6, ..=20) => "Gemini",
        (6, 21..) | (7, ..=22) => "Cancer",
        (7, 23..) | (8, ..=22) => "Leo",
        (8, 23..) | (9, ..=22) => "Virgo",
        (9, 23..) | (10, ..=22) => "Libra",
        (10, 23..) | (11, ..=21) => "Scorpio",
        (11, 22..) | (12, ..=21) => "Sagittarius",
        (12, 22..) | (1, ..=19) => "Capricorn",
        _ => "Invalid date",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Expect the date as MM/DD
    if args.len() != 2 {
        println!("Usage: {} MM/DD", args[0]);
        return;
    }

    let parts: Vec<&str> = args[1].split('/').collect();
    if parts.len() != 2 {
        println!("Please enter the date in MM/DD format.");
        return;
    }

    // Parse month and day from the input string.
    let month: u32 = parts[0].trim().parse().unwrap_or(0);
    let day: u32 = match parts[1].trim().parse() {
        Ok(d) => d,
        Err(m) => {
            println!("Error: {}", m);
            return;
        }
    };

    // Validate the date using chrono's NaiveDate.
    if NaiveDate::from_ymd_opt(2024, month, day).is_none() {
        println!("Invalid date");
        return;
    }
    let sign = zodiac_sign(month, day);
    println!("{}", sign);
}
// Add to Cargo.toml: chrono = "0.4"
