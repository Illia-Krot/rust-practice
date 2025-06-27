use std::io::{self, BufRead};

/*
 * Complete the 'equalizeArray' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn equalizeArray(arr: &[i32]) -> i32 {
    let mut freq = std::collections::HashMap::new();
    for &x in arr {
        *freq.entry(x).or_insert(0) += 1;
    }
    let max_count = freq.values().cloned().max().unwrap_or(0);
    (arr.len() as i32) - max_count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let arr: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = equalizeArray(&arr);
    println!("{}", result);
}
