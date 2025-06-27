use std::io::{self, BufRead};

/*
 * Complete the 'alternate' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING s as parameter.
 */

fn alternate(s: &str) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let unique: Vec<char> = {
        let mut u = chars.clone();
        u.sort_unstable();
        u.dedup();
        u
    };

    let mut max_len = 0;
    for i in 0..unique.len() {
        for j in i+1..unique.len() {
            let a = unique[i];
            let b = unique[j];
            let mut prev = None;
            let mut len = 0;
            let mut valid = true;
            for &c in &chars {
                if c == a || c == b {
                    if Some(c) == prev {
                        valid = false;
                        break;
                    }
                    prev = Some(c);
                    len += 1;
                }
            }
            if valid {
                max_len = max_len.max(len);
            }
        }
    }
    max_len
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let s = it.next().unwrap().unwrap();

    let result = alternate(&s);
    println!("{}", result);
}
