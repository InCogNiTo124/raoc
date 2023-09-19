use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let result = buffer
        .trim()
        .split('\n')
        .map(|row| {
            let numbers = row.split('\t').map(|t| t.parse::<u32>().unwrap());
            numbers.clone().max().unwrap() - numbers.min().unwrap()
        })
        .sum::<u32>();
    println!("{}", result);
}
