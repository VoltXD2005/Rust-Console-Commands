use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{}", args[1].replace(" ", ""));
    } else if args.len() == 3 && args[1] == "-f" {
        let file = File::open(&args[2])?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        println!("{}", content.replace(" ", ""));
    } else {
        eprintln!("Usage: strip-spaces <string> OR strip-spaces -f <filename>");
    }
    Ok(())
}
