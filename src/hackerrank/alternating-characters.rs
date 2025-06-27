use std::io::{self, BufRead};

/*
 * Complete the 'alternatingCharacters' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn alternatingCharacters(s: &str) -> i32 {
    let mut removals = 0;
    let mut prev = '\0';
    for ch in s.chars() {
        if ch == prev {
            removals += 1;
        } else {
            prev = ch;
        }
    }
    removals
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let q = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    for _ in 0..q {
        let s = lines.next().unwrap().unwrap();
        let result = alternatingCharacters(&s);
        println!("{}", result);
    }
}
