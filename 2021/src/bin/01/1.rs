use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);

    let count = buffer
        .trim()
        .split('\n')
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|a| a.get(1) > a.get(0))
        .count();
    println!("{}", count);
}
