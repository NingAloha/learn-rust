pub fn calculate_price(apple: i32) -> i32 {
    if apple < 40 {
        apple * 2
    } else {
        apple * 1
    }
}
