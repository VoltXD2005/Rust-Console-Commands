use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: repeat <string> <count>");
        std::process::exit(1);
    }
    let count: usize = args[2].parse().expect("Invalid count");
    for _ in 0..count {
        print!("{}", args[1]);
    }
    println!();
}
