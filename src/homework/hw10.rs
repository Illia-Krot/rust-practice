pub fn is_palindrome(num: i32) -> bool {
    // Конвертуємо число в рядок
    let s = num.to_string();
    // Порівнюємо рядок з його реверсом
    s == s.chars().rev().collect::<String>()
}
