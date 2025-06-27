use std::io::{self, BufRead};

/*
 * Complete the 'birthday' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY s
 *  2. INTEGER d
 *  3. INTEGER m
 */

fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    let mut count = 0;
    let m_usize = m as usize;
    for i in 0..=s.len().saturating_sub(m_usize) {
        let sum: i32 = s[i..i + m_usize].iter().sum();
        if sum == d {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let s: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let dm: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let d = dm[0];
    let m = dm[1];

    let result = birthday(&s, d, m);
    println!("{}", result);
}
