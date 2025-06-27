use std::io::{self, BufRead};

/*
 * Complete the 'marsExploration' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn marsExploration(s: &str) -> i32 {
    let mut count = 0;
    for (i, ch) in s.chars().enumerate() {
        let expected = match i % 3 {
            0 | 2 => 'S',
            1 => 'O',
            _ => unreachable!(),
        };
        if ch != expected {
            count += 1;
        }
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let s = it.next().unwrap().unwrap();

    let result = marsExploration(&s);
    println!("{}", result);
}
