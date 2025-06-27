use std::io::{self, BufRead};

/*
 * Complete the 'countApplesAndOranges' function below.
 *
 * The function accepts following parameters:
 *  1. INTEGER s: начало дома
 *  2. INTEGER t: конец дома
 *  3. INTEGER a: место расположения яблони
 *  4. INTEGER b: место расположения вишнёвы (апельсиновой) деревни
 *  5. INTEGER_ARRAY apples: расстояния, которые падают яблоки от яблони
 *  6. INTEGER_ARRAY oranges: расстояния, которые падают апельсины от апельсинового дерева
 */

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apples_count = apples.iter()
        .filter(|&&d| {
            let pos = a + d;
            pos >= s && pos <= t
        })
        .count();

    let oranges_count = oranges.iter()
        .filter(|&&d| {
            let pos = b + d;
            pos >= s && pos <= t
        })
        .count();

    println!("{}", apples_count);
    println!("{}", oranges_count);
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let st: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let s = st[0];
    let t = st[1];

    let ab: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let a = ab[0];
    let b = ab[1];

    let _mn: Vec<usize> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let apples: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let oranges: Vec<i32> = it.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    countApplesAndOranges(s, t, a, b, &apples, &oranges);
}
