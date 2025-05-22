/*
.\ascii-art.exe Rust
# Output:
#  RRR  U U  SSS  TTT 
#  R R  U U  S     T  
#  RR   U U   S    T  
#  R R  U U     S  T  
#  R R   UU  SS   T  
*/

use std::env;

const BLOCKS: [&str; 26] = [
    " A \nA A\nAAA\nA A\nA A", // A
    "BB \nB B\nBB \nB B\nBB ", // B
    " CC\nC  \nC  \nC  \n CC", // C
    "DD \nD D\nD D\nD D\nDD ", // D
    "EEE\nE  \nEE \nE  \nEEE", // E
    "FFF\nF  \nFF \nF  \nF  ", // F
    " GG\nG  \nG G\nG G\n GG", // G
    "H H\nH H\nHHH\nH H\nH H", // H
    "III\n I \n I \n I \nIII", // I
    " JJ\n  J\n  J\nJ J\n JJ", // J
    "K K\nKK \nK  \nKK \nK K", // K
    "L  \nL  \nL  \nL  \nLLL", // L
    "M M\nMMM\nM M\nM M\nM M", // M
    "N N\nNNN\nN N\nN N\nN N", // N
    " O \nO O\nO O\nO O\n O ", // O
    "PP \nP P\nPP \nP  \nP  ", // P
    " QQ\nQ Q\nQ Q\n QQ\n  Q", // Q
    "RR \nR R\nRR \nR R\nR R", // R
    " SS\nS  \n S \n  S\nSS ", // S
    "TTT\n T \n T \n T \n T ", // T
    "U U\nU U\nU U\nU U\n UU", // U
    "V V\nV V\nV V\nV V\n V ", // V
    "W W\nW W\nW W\nWWW\nW W", // W
    "X X\nX X\n X \nX X\nX X", // X
    "Y Y\nY Y\n Y \n Y \n Y ", // Y
    "ZZZ\n  Z\n Z \nZ  \nZZZ", // Z
];

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ascii-art <text>");
        std::process::exit(1);
    }
    let text = args[1].to_uppercase();
    let mut lines = vec![String::new(); 5];
    for ch in text.chars() {
        if ch >= 'A' && ch <= 'Z' {
            let block = BLOCKS[(ch as usize) - ('A' as usize)];
            for (i, line) in block.lines().enumerate() {
                lines[i].push_str(line);
                lines[i].push(' ');
            }
        } else {
            for l in &mut lines {
                l.push_str("    ");
            }
        }
    }
    for line in lines {
        println!("{}", line);
    }
}