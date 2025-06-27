use std::io::{self, BufRead};

/*
 * Complete the 'countingValleys' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts the following parameters:
 *  1. INTEGER steps
 *  2. STRING path
 */

fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut level = 0;
    let mut valleys = 0;
    for c in path.chars() {
        if c == 'U' {
            level += 1;
            if level == 0 {
                valleys += 1;
            }
        } else if c == 'D' {
            level -= 1;
        }
    }
    valleys
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let steps = it.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let path = it.next().unwrap().unwrap();

    let result = countingValleys(steps, &path);
    println!("{}", result);
}
