use std::io::{self, BufRead};

/*
 * Complete the 'migratoryBirds' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for &id in arr {
        *counts.entry(id).or_insert(0) += 1;
    }
    let mut max_id = i32::MAX;
    let mut max_count = 0;
    for (&id, &cnt) in &counts {
        if cnt > max_count || (cnt == max_count && id < max_id) {
            max_count = cnt;
            max_id = id;
        }
    }
    max_id
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = migratoryBirds(&arr);
    println!("{}", result);
}
