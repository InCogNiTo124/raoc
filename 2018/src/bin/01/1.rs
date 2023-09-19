use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let result = buffer
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .reduce(|accum, elem| accum + elem)
        .unwrap();
    println!("{}", result);
}
