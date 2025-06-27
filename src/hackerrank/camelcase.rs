use std::io::{self, BufRead};

/*
 * Complete the 'camelcase' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn camelcase(s: &str) -> i32 {
    let mut count = 1;
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let s = it.next().unwrap().unwrap();
    let result = camelcase(&s);
    println!("{}", result);
}
