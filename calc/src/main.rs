/*
.\calc.exe 2 + 3 * 4
# Output:
# 14

.\calc.exe "(10 - 2) / 2"
# Output:
# 4
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: calc <expression>");
        std::process::exit(1);
    }
    let expr = args[1..].join(" ");
    match meval::eval_str(expr) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
}

// Add to Cargo.toml dependencies:
// meval = "0.2"