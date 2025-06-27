use std::io::{self, BufRead};

/*
 * Complete the 'cutTheSticks' function below.
 *
 * The function is expected to return INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn cutTheSticks(arr: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut sticks: Vec<i32> = arr.to_vec();
    sticks.sort_unstable();

    while !sticks.is_empty() {
        result.push(sticks.len() as i32);
        let min_len = sticks[0];
        sticks = sticks.into_iter().map(|x| x - min_len).filter(|&x| x > 0).collect();
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let arr: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = cutTheSticks(&arr);
    for val in result {
        println!("{}", val);
    }
}
