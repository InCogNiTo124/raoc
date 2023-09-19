use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let result = buffer
        .trim()
        .split('\n')
        .map(|row| {
            let numbers = row
                .split('\t')
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            for i in 0..(numbers.len() - 1) {
                let a = numbers[i];
                for j in (i+1)..numbers.len() {
                    let b = numbers[j];
                    let aa = if a > b { a } else { b };
                    let bb = if a > b { b } else { a };
                    let result = aa / bb;
                    if result * bb == aa {
                        return result;
                    }
                }
            }
            return 0; // error
        })
        .sum::<u32>();
    println!("{}", result);
}
