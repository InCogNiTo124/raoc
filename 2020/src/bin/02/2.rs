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

            let min_contains = password.chars().nth(min-1).unwrap() == c;
            let max_contains = password.chars().nth(max-1).unwrap() == c;
            (min_contains ^ max_contains) as u32
        })
        .sum::<u32>();
    println!("{}", number);
}
