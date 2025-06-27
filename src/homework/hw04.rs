fn draw_rhombus(n: usize) {
    let mut result = String::new();

    // upper half including middle
    for i in 0..n {
        // spaces before stars
        let spaces = n - i - 1;
        // stars count (2*i + 1)
        let stars = 2 * i + 1;

        // add spaces
        for _ in 0..spaces {
            result.push(' ');
        }
        // add stars
        for _ in 0..stars {
            result.push('*');
        }
        result.push('\n');
    }

    // lower half
    for i in (0..n-1).rev() {
        let spaces = n - i - 1;
        let stars = 2 * i + 1;

        for _ in 0..spaces {
            result.push(' ');
        }
        for _ in 0..stars {
            result.push('*');
        }
        result.push('\n');
    }

    // print the entire rhombus at once
    println!("{}", result);
}

fn main() {
    draw_rhombus(3);
}
