use std::io::{self, BufRead};

/*
 * Complete the 'bonAppetit' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY bill
 *  2. INTEGER k
 *  3. INTEGER b
 */

fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let total: i32 = bill.iter().sum();
    let anna_share = (total - bill[k as usize]) / 2;
    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let first: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let _n = first[0];
    let k = first[1];

    let bill: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let b = it.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    bonAppetit(&bill, k, b);
}
