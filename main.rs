use std::io;
use chrono::Local;

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    println!("1: F to C");
    println!("2: C to F");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let mut temp = String::new();
    println!("Enter temperature:");
    io::stdin().read_line(&mut temp).unwrap();

    let choice: u32 = choice.trim().parse().unwrap();
    let temp: f64 = temp.trim().parse().unwrap();

    let now = Local::now();

    if choice == 1 {
        println!("Result: {}", f_to_c(temp));
    } else {
        println!("Result: {}", c_to_f(temp));
    }

    println!("Time: {}", now);
}