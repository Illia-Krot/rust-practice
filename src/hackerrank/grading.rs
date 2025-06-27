use std::io::{self, BufRead};

/*
 * Complete the 'gradingStudents' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY grades as parameter.
 */

fn gradingStudents(grades: &[i32]) -> Vec<i32> {
    grades
        .iter()
        .map(|&grade| {
            if grade < 38 {
                grade
            } else {
                let next_mult = ((grade / 5) + 1) * 5;
                if next_mult - grade < 3 {
                    next_mult
                } else {
                    grade
                }
            }
        })
        .collect()
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let _n = it.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let grades: Vec<i32> = it.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();

    let result = gradingStudents(&grades);
    for (i, g) in result.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", g);
    }
    println!();
}
