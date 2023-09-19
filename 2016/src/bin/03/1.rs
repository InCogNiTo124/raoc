use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");

    let result = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let parts = line
                .split(' ')
                .filter(|t| t.len() > 0)
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1], parts[2])
        })
        .filter(|(a, b, c)| *a < (b + c) && *b < (a + c) && *c < (a + b))
        .count();
    println!("{}", result);
}
