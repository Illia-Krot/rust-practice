use std::io::{self, BufRead};

/*
 * Complete the 'beautifulBinaryString' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING b as parameter.
 */

fn beautifulBinaryString(b: &str) -> i32 {
    let mut count = 0;
    let mut i = 0;
    let bytes = b.as_bytes();
    let n = bytes.len();

    while i + 2 < n {
        if &bytes[i..i+3] == b"010" {
            count += 1;
            i += 3; // shift past the replaced substring
        } else {
            i += 1;
        }
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();
    let b = it.next().unwrap().unwrap();

    let result = beautifulBinaryString(&b);
    println!("{}", result);
}
