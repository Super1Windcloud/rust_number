use rust_number::{divide, divide_all, minus, plus, plus_all, round, strip, times, times_all};

fn main() {
    println!(
        "strip(0.09999999999999998, 15) = {}",
        strip(0.09999999999999998_f64, 15)
    );
    println!("plus(0.1, 0.2) = {}", plus(0.1, 0.2));
    println!("minus(1.0, 0.9) = {}", minus(1.0, 0.9));
    println!("times(3, 0.3) = {}", times(3, 0.3));
    println!("divide(1.21, 1.1) = {}", divide(1.21, 1.1));
    println!("round(0.105, 2) = {}", round(0.105, 2));
    println!("plus_all([0.1, 0.2, 0.3]) = {}", plus_all([0.1, 0.2, 0.3]));
    println!(
        "times_all([0.1, 0.2, 10.0]) = {}",
        times_all([0.1, 0.2, 10.0])
    );
    println!(
        "divide_all([1.2, 0.3, 2.0]) = {}",
        divide_all([1.2, 0.3, 2.0])
    );
}
