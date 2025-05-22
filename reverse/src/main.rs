/*
.\reverse.exe "hello world"
# Output:
# dlrow olleh

# lines.txt contains:
# hello
# world

.\reverse.exe -f lines.txt
# Output:
# olleh
# dlrow
*/
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // Reverse string argument
        println!("{}", args[1].chars().rev().collect::<String>());
    } else if args.len() == 3 && args[1] == "-f" {
        // Reverse each line in file
        let file = File::open(&args[2])?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let l = line?;
            println!("{}", l.chars().rev().collect::<String>());
        }
    } else {
        eprintln!("Usage: reverse <string> OR reverse -f <filename>");
    }
    Ok(())
}