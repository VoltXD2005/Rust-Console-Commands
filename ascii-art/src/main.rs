/*
This is a simple Rust program that prints ASCII art for a given string.
It demonstrates how to read command-line arguments, use arrays, and print stylized output.

Example usage:
    .\ascii-art.exe ECU
Output:
EEE  CC U U 
E   C   U U 
EE  C   U U
E   C   U U
EEE  CC  UU 
*/

// Import the 'env' module to access command-line arguments.
use std::env;

// Define a constant array of ASCII art blocks for each uppercase letter A-Z.
// Each entry is a multi-line string representing the letter in ASCII art.
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
    // Collect all command-line arguments into a vector of strings.
    // args[0] is the program name, args[1] is the text to convert to ASCII art.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // Print usage instructions if the argument count is incorrect.
        eprintln!("Usage: ascii-art <text>");
        std::process::exit(1);
    }
    // Convert the input text to uppercase so it matches the BLOCKS array.
    let text = args[1].to_uppercase();

    // Prepare a vector to hold each line of the final ASCII art output.
    // There are 5 lines in each letter block.
    let mut lines = vec![String::new(); 5];

    // For each character in the input text:
    for ch in text.chars() {
        // If the character is an uppercase letter A-Z, get its ASCII art block.
        if ch >= 'A' && ch <= 'Z' {
            let block = BLOCKS[(ch as usize) - ('A' as usize)];
            // Add each line of the block to the corresponding output line.
            for (i, line) in block.lines().enumerate() {
                lines[i].push_str(line);
                lines[i].push(' '); // Add a space between letters.
            }
        } else {
            // For non-letter characters, add spaces to keep alignment.
            for l in &mut lines {
                l.push_str("    ");
            }
        }
    }
    // Print each line of the ASCII art.
    for line in lines {
        println!("{}", line);
    }
}