use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let result = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let digits: Vec<_> = line
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum::<u32>();
    println!("{}", result);
}
