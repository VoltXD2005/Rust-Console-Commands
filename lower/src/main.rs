use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{}", args[1].to_lowercase());
    } else if args.len() == 3 && args[1] == "-f" {
        let file = File::open(&args[2])?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line?.to_lowercase());
        }
    } else {
        eprintln!("Usage: lower <string> OR lower -f <filename>");
    }
    Ok(())
}
