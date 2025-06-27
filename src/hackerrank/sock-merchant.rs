use std::io::{self, BufRead};

/*
 * Complete the 'sockMerchant' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER_ARRAY ar
 */

fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let mut freq = std::collections::HashMap::new();
    for &sock in ar {
        *freq.entry(sock).or_insert(0) += 1;
    }
    freq.values().map(|&count| count / 2).sum()
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let n = it.next().unwrap().unwrap()
        .trim().parse::<i32>().unwrap();

    let ar: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", sockMerchant(n, &ar));
}
