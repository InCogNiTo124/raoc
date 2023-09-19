use scanf::sscanf;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let number = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut min: usize = 0;
            let mut max: usize = 0;
            let mut c: char = 'a';
            let mut password: String = String::new();

            let _ = sscanf!(line, "{}-{} {}: {}", min, max, c, password);

            let count = password.chars().map(|t| (t == c) as usize).sum::<usize>();
            (min <= count && count <= max) as u32
        })
        .sum::<u32>();
    println!("{}", number);
}
