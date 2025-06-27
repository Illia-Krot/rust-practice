use std::io::{self, BufRead};

/*
 * Complete the 'jumpingOnClouds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY c as parameter.
 */

fn jumpingOnClouds(c: &[i32]) -> i32 {
    let n = c.len();
    let mut i = 0;
    let mut jumps = 0;
    while i + 2 < n {
        if c[i + 2] == 0 {
            i += 2;
        } else {
            i += 1;
        }
        jumps += 1;
    }
    if i + 1 == n - 1 {
        jumps += 1;
    }
    jumps
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let c: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = jumpingOnClouds(&c);
    println!("{}", result);
}
