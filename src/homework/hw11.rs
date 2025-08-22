use rand::Rng;

// Генеруємо вектор з n елементів у діапазоні [10..99]
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Пошук мінімальної пари сусідніх елементів
// Повертає Some((індекс першого елемента, сума)) або None, якщо неможливо знайти
pub fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32)> {
    if data.len() < 2 {
        return None;
    }
    let mut min_index = 0;
    let mut min_sum = data[0] + data[1];
    for i in 1..data.len() - 1 {
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
            min_index = i;
        }
    }
    Some((min_index, min_sum))
}

// Функція виводу в консоль
pub fn print_vector_with_min_pair(data: &[i32]) {
    // Вивід індексів
    print!("indexes:");
    for i in 0..data.len() {
        print!(" {:>2}.", i);
    }
    println!();

    // Вивід даних
    print!("data:   [");
    for (i, val) in data.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", val);
    }
    println!("]");

    if let Some((idx, sum)) = min_adjacent_sum(data) {
        // Створюємо рядок зі стрілочками \__ __/
        let mut pointers = String::from("indexes:");
        for i in 0..data.len() {
            if i == idx {
                pointers.push_str(r"\__");
            } else if i == idx + 1 {
                pointers.push_str(" __/");
            } else {
                pointers.push_str("   ");
            }
        }
        println!("{}", pointers);

        println!(
            "min adjacent sum={}+{}={} at indexes:{},{}",
            data[idx], data[idx + 1], sum, idx, idx + 1
        );
    } else {
        println!("Data too short to find adjacent pairs.");
    }
}

fn main() {
    let data = gen_random_vector(20);
    print_vector_with_min_pair(&data);
}

