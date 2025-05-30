use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: lower [--trim] <string...>");
        std::process::exit(1);
    }

    let mut trim = false;
    let mut input_words = vec![];

    for arg in &args[1..] {
        if arg == "--trim" {
            trim = true;
        } else {
            input_words.push(arg.as_str());
        }
    }

    if input_words.is_empty() {
        eprintln!("Error: No input string provided.");
        std::process::exit(1);
    }

    let input = input_words.join(" ");
    let processed = if trim {
        input.trim().to_lowercase()
    } else {
        input.to_lowercase()
    };

    println!("{}", processed);
}
