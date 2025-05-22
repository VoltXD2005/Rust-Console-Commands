use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{}", args[1].to_uppercase());
    } else if args.len() == 3 && args[1] == "-f" {
        let file = File::open(&args[2])?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line?.to_uppercase());
        }
    } else {
        eprintln!("Usage: upper <string> OR upper -f <filename>");
    }
    Ok(())
}
