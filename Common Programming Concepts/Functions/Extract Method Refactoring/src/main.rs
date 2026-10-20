fn main() {
    let mut year = 1993;
    print_ten_years_ago(year);

    year = 2021;
    print_ten_years_ago(year);
}

fn print_ten_years_ago(year: i32) {
    println!("{}: 10 years ago was {}", year, year - 10);
}