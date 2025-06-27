use std::io::{self, BufRead};

/*
 * Complete the 'pangrams' function below.
 *
 * The function accepts STRING s as parameter.
 * The function is expected to return a STRING: "pangram" или "not pangram".
 */

fn pangrams(s: &str) -> String {
    let mut seen = [false; 26];
    let mut count = 0;

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            let idx = c.to_ascii_lowercase() as usize - 'a' as usize;
            if !seen[idx] {
                seen[idx] = true;
                count += 1;
                if count == 26 {
                    return "pangram".to_string();
                }
            }
        }
    }

    if count == 26 {
        "pangram".to_string()
    } else {
        "not pangram".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let s = stdin.lock().lines().next().unwrap().unwrap();

    let result = pangrams(&s);
    println!("{}", result);
}
