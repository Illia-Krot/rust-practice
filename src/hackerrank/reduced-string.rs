use std::io::{self, BufRead};

/// Complete the 'superReducedString' function below.
/// The function accepts STRING s as parameter,
/// and returns STRING: силащью редуцированная строка или "Empty String".
fn superReducedString(s: &str) -> String {
    let mut stack = Vec::with_capacity(s.len());
    for ch in s.chars() {
        if stack.last() == Some(&ch) {
            stack.pop();
        } else {
            stack.push(ch);
        }
    }
    if stack.is_empty() {
        "Empty String".into()
    } else {
        stack.iter().collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();
    let s = it.next().unwrap().unwrap();
    let result = superReducedString(&s);
    println!("{}", result);
}
