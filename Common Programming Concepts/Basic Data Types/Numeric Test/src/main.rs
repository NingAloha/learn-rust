fn main() {
    let fahrenheit_degree = 451_f64;
    let celsius_degree = (fahrenheit_degree - 32_f64) * 0.5556;
    println!("{}°F is {:.1}°C", fahrenheit_degree, celsius_degree);
}
