use std::io::{self, BufRead};

/*
 * Complete the 'minimumNumber' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. STRING password
 */

fn minimumNumber(n: i32, password: &str) -> i32 {
    let mut missing_types = 0;
    if !password.chars().any(|c| c.is_ascii_lowercase()) { missing_types += 1; }
    if !password.chars().any(|c| c.is_ascii_uppercase()) { missing_types += 1; }
    if !password.chars().any(|c| c.is_ascii_digit())      { missing_types += 1; }
    if !password.chars().any(|c| "!@#$%^&*()-+".contains(c)) { missing_types += 1; }
    std::cmp::max(missing_types, 6 - n)
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let n = it.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let password = it.next().unwrap().unwrap();

    let result = minimumNumber(n, &password);
    println!("{}", result);
}
