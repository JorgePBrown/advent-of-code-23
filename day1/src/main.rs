use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut s = String::new();

    let mut v: u32 = 0;

    while let Ok(n_bytes) = stdin.read_to_string(&mut s) {
        if n_bytes == 0 { break }
        for line in s.lines() {
            let first_digit: u32 = get_first_digit(&line).expect(&format!("There to be a first digit for line {line}")).to_digit(10).unwrap();
            let last_digit: u32 = get_last_digit(&line).expect(&format!("There to be a last digit for line {line}")).to_digit(10).unwrap();

            v += first_digit * 10 + last_digit;
        }
        s.clear();
    };
    
    println!("{v}");
}

fn get_first_digit(input: &str) -> Option<char> {
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            return Some(c);
        }
    }
    return None;
}
fn get_last_digit(input: &str) -> Option<char> {
    for c in input.chars().rev() {
        if c >= '0' && c <= '9' {
            return Some(c);
        }
    }
    return None;
}
