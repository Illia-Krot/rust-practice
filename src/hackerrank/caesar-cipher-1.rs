use std::io::{self, BufRead};

/*
 * Complete the 'caesarCipher' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. INTEGER k
 */

fn caesarCipher(s: &str, k: i32) -> String {
    let shift = (k % 26 + 26) % 26;
    s.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                let c2 = ((c as u8 - base + shift as u8) % 26) + base;
                c2 as char
            } else if c.is_ascii_uppercase() {
                let base = b'A';
                let c2 = ((c as u8 - base + shift as u8) % 26) + base;
                c2 as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let s = it.next().unwrap().unwrap();
    let k = it.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let result = caesarCipher(&s, k);
    println!("{}", result);
}
