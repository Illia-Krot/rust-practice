use std::io::{self, BufRead};

/*
 * Complete the 'libraryFine' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER d1 — день возврата
 *  2. INTEGER m1 — месяц возврата
 *  3. INTEGER y1 — год возврата
 *  4. INTEGER d2 — день ожидаемого возврата
 *  5. INTEGER m2 — месяц ожидаемого возврата
 *  6. INTEGER y2 — год ожидаемого возврата
 */

fn libraryFine(d1: i32, m1: i32, y1: i32, d2: i32, m2: i32, y2: i32) -> i32 {
    if y1 > y2 {
        10000
    } else if y1 < y2 {
        0
    } else if m1 > m2 {
        500 * (m1 - m2)
    } else if m1 < m2 {
        0
    } else if d1 > d2 {
        15 * (d1 - d2)
    } else {
        0
    }
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    // Считываем дату фактического возврата
    let actual: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let d1 = actual[0];
    let m1 = actual[1];
    let y1 = actual[2];

    // Считываем ожидаемую дату возврата
    let expected: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let d2 = expected[0];
    let m2 = expected[1];
    let y2 = expected[2];

    let result = libraryFine(d1, m1, y1, d2, m2, y2);
    println!("{}", result);
}
