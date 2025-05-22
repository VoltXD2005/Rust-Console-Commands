use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{}", args[1].chars().count());
    } else if args.len() == 3 && args[1] == "-f" {
        let file = File::open(&args[2])?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        println!("{}", content.chars().count());
    } else {
        eprintln!("Usage: count-chars <string> OR count-chars -f <filename>");
    }
    Ok(())
}
