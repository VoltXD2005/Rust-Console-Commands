// This is a simple Rust program that waits for a specified number of seconds.
// It demonstrates how to read command-line arguments and use timers.

use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    // args[0] is the program name, args[1] is the number of seconds to wait.
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: timer <seconds>");
        std::process::exit(1);
    }

    let secs: u64 = args[1].parse().expect("Please provide a valid number of seconds");
    println!("Waiting for {} seconds...", secs);
    thread::sleep(Duration::from_secs(secs));
    println!("Done!");
}