use std::io::{self, BufRead};

/*
 * Complete the 'breakingRecords' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY scores as parameter.
 */

fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    let mut best = scores[0];
    let mut worst = scores[0];
    let mut best_breaks = 0;
    let mut worst_breaks = 0;

    for &score in &scores[1..] {
        if score > best {
            best = score;
            best_breaks += 1;
        } else if score < worst {
            worst = score;
            worst_breaks += 1;
        }
    }

    vec![best_breaks, worst_breaks]
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let scores: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = breakingRecords(&scores);
    println!("{} {}", result[0], result[1]);
}
