use std::io::{self, BufRead};

/*
 * Complete the 'pageCount' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n (количество страниц в книге)
 *  2. INTEGER p (целевая страница)
 */

fn pageCount(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = n / 2 - p / 2;
    std::cmp::min(from_front, from_back)
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let n = it.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let p = it.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = pageCount(n, p);
    println!("{}", result);
}
