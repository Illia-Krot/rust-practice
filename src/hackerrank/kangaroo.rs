use std::io::{self, BufRead};

/*
 * Complete the 'kangaroo' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER x1
 *  2. INTEGER v1
 *  3. INTEGER x2
 *  4. INTEGER v2
 */

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        return "NO".to_string();
    }
    let dist = x2 - x1;
    let jump_diff = v1 - v2;
    if dist % jump_diff == 0 {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let vals: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let result = kangaroo(vals[0], vals[1], vals[2], vals[3]);
    println!("{}", result);
}
