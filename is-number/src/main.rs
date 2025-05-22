use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: is-number <string>");
        std::process::exit(1);
    }
    let s = &args[1];
    let is_num = s.parse::<f64>().is_ok();
    println!("{}", is_num);
}
