use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: count-lines <filename> [--skip-comments]");
        std::process::exit(1);
    }

    let filename = &args[1];
    let skip_comments = args.len() == 3 && args[2] == "--skip-comments";

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if skip_comments && trimmed.starts_with("//") {
            continue;
        }

        count += 1;
    }

    println!("Line count: {}", count);
    Ok(())
}
