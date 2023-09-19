
use std::io::{self, Read};

fn check(a: &u32, b: &u32, c: &u32) -> bool {
    *a < (b+c) && *b < (a+c) && *c < (a+b)
}

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");

    let mut result = 0;
    let temp = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let parts = line
                .split(' ')
                .filter(|t| t.len() > 0)
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1], parts[2])
        }).collect::<Vec<_>>();
        for i in (0..temp.len()).step_by(3) {
            result += check(&temp[i].0, &temp[i+1].0, &temp[i+2].0) as u32;
            result += check(&temp[i].1, &temp[i+1].1, &temp[i+2].1) as u32;
            result += check(&temp[i].2, &temp[i+1].2, &temp[i+2].2) as u32;
        }
        println!("{}", result);
}

