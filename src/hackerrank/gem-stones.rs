use std::io::{self, BufRead};

/*
 * Complete the 'gemstones' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts STRING_ARRAY arr as parameter.
 */

fn gemstones(arr: &[String]) -> i32 {
    if arr.is_empty() {
        return 0;
    }
    let mut common = std::collections::HashSet::new();
    arr[0].chars().for_each(|c| {
        if c.is_ascii_lowercase() {
            common.insert(c);
        }
    });
    for rock in &arr[1..] {
        let s: std::collections::HashSet<char> = rock.chars().filter(|c| c.is_ascii_lowercase()).collect();
        common = common.intersection(&s).cloned().collect();
        if common.is_empty() {
            break;
        }
    }
    common.len() as i32
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let arr: Vec<String> = it.take(_n).map(|line| line.unwrap()).collect();

    let result = gemstones(&arr);
    println!("{}", result);
}
