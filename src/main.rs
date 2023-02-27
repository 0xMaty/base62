use std::convert::TryFrom;
use std::io;

const BASE: usize = 62;
const ALPHABET: [&str; BASE] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h", "i",
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B",
    "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
    "V", "W", "X", "Y", "Z",
];

fn main() {
    // Get number.
    println!("Enter the decimal number you want to convert to base62:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");
    let number: u128 = number.trim().parse().expect("Failed to parse number.");

    // Convert to base62.
    let encoded = to_base62(number);
    println!("{number} is represented as: {encoded}");
}

fn to_base62(number: u128) -> String {
    let mut value = usize::try_from(number)
        .ok()
        .expect("Failed to convert to usize! Try smaller number.");
    let mut encoded = String::new();
    while value > 0 {
        let remainder = value % BASE;
        encoded.insert_str(0, ALPHABET[remainder]);
        value /= BASE;
    }
    return encoded;
}
