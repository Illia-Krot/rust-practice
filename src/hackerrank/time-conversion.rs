use std::io::{self, BufRead};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let hour = &s[0..2];
    let rest = &s[2..8]; // :mm:ss
    let period = &s[8..]; // AM PM
    let h: u32 = hour.parse().unwrap();

    let new_hour = match period {
        "AM" => {
            if h == 12 {
                "00".to_string()
            } else {
                format!("{:02}", h)
            }
        }
        "PM" => {
            if h == 12 {
                "12".to_string()
            } else {
                format!("{:02}", h + 12)
            }
        }
        _ => hour.to_string(), 
    };

    format!("{}{}", new_hour, rest)
}

fn main() {
    let stdin = io::stdin();
    let mut it = stdin.lock().lines();

    let s = it.next().unwrap().unwrap();

    let result = timeConversion(&s);

    println!("{}", result);
}
