use std::io::{self, BufRead};

/*
 * Complete the 'divisibleSumPairs' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER k
 *  3. INTEGER_ARRAY ar
 */

fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    for i in 0..n as usize {
        for j in i + 1..n as usize {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let first: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = first[0];
    let k = first[1];

    let ar: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = divisibleSumPairs(n, k, &ar);
    println!("{}", result);
}
