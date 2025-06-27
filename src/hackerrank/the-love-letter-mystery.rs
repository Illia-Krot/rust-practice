use std::io::{self, BufRead};

/*
 * Complete the 'theLoveLetterMystery' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn theLoveLetterMystery(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut count = 0;
    for i in 0..n/2 {
        count += (chars[i] as i32 - chars[n - 1 - i] as i32).abs();
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let q = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    for _ in 0..q {
        let s = lines.next().unwrap().unwrap();
        let result = theLoveLetterMystery(&s);
        println!("{}", result);
    }
}
