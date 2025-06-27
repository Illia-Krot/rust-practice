use std::io::{self, BufRead};

/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut max_height = candles[0];
    let mut count = 0;

    for &h in candles {
        if h > max_height {
            max_height = h;
            count = 1;
        } else if h == max_height {
            count += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let candles: Vec<i32> = it.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);
    println!("{}", result);
}
