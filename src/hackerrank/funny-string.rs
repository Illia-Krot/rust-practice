use std::io::{self, BufRead};

/*
 * Complete the 'funnyString' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn funnyString(s: &str) -> String {
    let bytes = s.as_bytes();
    let rev = bytes.iter().rev().cloned().collect::<Vec<_>>();
    let n = bytes.len();
    for i in 1..n {
        if (bytes[i] as i32 - bytes[i - 1] as i32).abs()
            != (rev[i] as i32 - rev[i - 1] as i32).abs()
        {
            return "Not Funny".to_string();
        }
    }
    "Funny".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let q = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    for _ in 0..q {
        let s = it.next().unwrap().unwrap();
        println!("{}", funnyString(&s));
    }
}
