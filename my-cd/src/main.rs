use std::env;

/*

This is a simple implementation of the `cd` command in Rust.

It can only be executed successfully from inside a custom shell

*/

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cd <directory>");
        std::process::exit(1);
    }
    if let Err(e) = env::set_current_dir(&args[1]) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("Error: {}", e),
    }
}