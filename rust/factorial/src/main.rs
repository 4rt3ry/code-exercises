use std::{
    io::{self, Write},
};

fn naive_factorial(n: u64) -> u64 {
    if n <= 0 { 1 } else { (1..=n).product() }
}

fn main() {
    let mut input_str = String::new();
    io::stdout()
        .write("Give me a positive integer: ".as_bytes())
        .expect("Error printing");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Error reading stdin");

    let factorial: u64 = input_str.trim().parse().expect("Failed to parse user input :(");

    println!("{}! = {}", input_str.trim(), naive_factorial(factorial));
}
