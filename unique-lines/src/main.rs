/* 
file.txt contains:
apple
banana
apple
orange

.\unique-lines.exe file.txt
Output:
    apple
    banana
    orange
*/

use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: unique-lines <filename>");
        std::process::exit(1);
    }
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut seen = HashSet::new();
    for line in reader.lines() {
        let line = line?;
        if seen.insert(line.clone()) {
            println!("{}", line);
        }
    }
    Ok(())
}