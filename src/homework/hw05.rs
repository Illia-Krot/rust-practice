fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs() 
}

fn main() {
    let x = 60;
    let y = 55;
    println!("GCD of {} and {} is {}", x, y, gcd(x, y));
}
