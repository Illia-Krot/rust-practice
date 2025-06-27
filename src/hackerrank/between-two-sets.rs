use std::io::{self, BufRead};

/*
 * Complete the 'getTotalX' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let mut l = a[0];
    for &x in &a[1..] {
        l = lcm(l, x);
    }

    let mut g = b[0];
    for &y in &b[1..] {
        g = gcd(g, y);
    }

    let mut count = 0;
    let mut multiple = l;
    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _nm: Vec<usize> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let a: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let b: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = getTotalX(&a, &b);
    println!("{}", result);
}
