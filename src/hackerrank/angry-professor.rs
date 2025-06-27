use std::io::{self, BufRead};

/*
 * Complete the 'angryProfessor' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY a
 */

fn angryProfessor(k: i32, a: Vec<i32>) -> String {
    let on_time = a.iter().filter(|&&time| time <= 0).count() as i32;
    if on_time >= k {
        "NO".to_string()
    } else {
        "YES".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    for _ in 0..t {
        let nk: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let _n = nk[0];
        let k = nk[1];

        let a: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let result = angryProfessor(k, a);
        println!("{}", result);
    }
}
