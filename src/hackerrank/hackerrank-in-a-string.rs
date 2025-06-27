use std::io::{self, BufRead};

/*
 * Complete the 'hackerrankInString' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn hackerrankInString(s: &str) -> String {
    let target = "hackerrank";
    let mut pos = 0;

    for c in s.chars() {
        if Some(c) == target.chars().nth(pos) {
            pos += 1;
            if pos == target.len() {
                return "YES".to_string();
            }
        }
    }

    "NO".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let q = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    for _ in 0..q {
        let s = lines.next().unwrap().unwrap();
        let result = hackerrankInString(&s);
        println!("{}", result);
    }
}
