use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let numbers = buffer.trim().split_whitespace().collect::<Vec<_>>();
    let n = numbers.len();
    let m = numbers[0].len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..m {
        let mut ones = numbers
            .iter()
            .filter(|line| line.chars().nth(i).unwrap() == '1')
            .count();
        if ones > n / 2 {
            gamma = gamma * 2 + 1;
            epsilon *= 2;
        } else {
            gamma *= 2;
            epsilon = epsilon * 2 + 1;
        }
    }
    println!("{}, {}, {}", gamma, epsilon, gamma * epsilon);
}
