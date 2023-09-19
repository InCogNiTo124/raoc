use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let mut numbers: Vec<u32> = buffer
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();
    numbers.sort_unstable();
    for i in 0..(numbers.len() - 2) {
        let a = numbers[i];
        for j in (i + 1)..(numbers.len() - 1) {
            let b = numbers[j];
            if a + b == 2020 {
                println!("{}", a * b);
            }
        }
    }
}
