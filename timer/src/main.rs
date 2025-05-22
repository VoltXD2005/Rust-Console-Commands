/*
.\timer.exe 5
# Output:
# 5...
# 4...
# 3...
# 2...
# 1...
# Time's up!
*/

use std::{env, thread, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: timer <seconds>");
        std::process::exit(1);
    }
    let seconds: u64 = args[1].parse().expect("Please enter a valid number");
    for i in (1..=seconds).rev() {
        println!("{}...", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Time's up!");
}