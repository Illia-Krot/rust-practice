use std::io::{self, BufRead};

/*
 * Complete the 'saveThePrisoner' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. INTEGER m
 *  3. INTEGER s
 */

fn saveThePrisoner(n: i32, m: i32, s: i32) -> i32 {
    // Обсчёт: (s - 1 + m - 1) % n + 1
    let last = (s - 1 + m - 1) % n + 1;
    if last <= 0 { last + n } else { last }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let vals: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = vals[0];
        let m = vals[1];
        let s = vals[2];

        let result = saveThePrisoner(n, m, s);
        println!("{}", result);
    }
}
