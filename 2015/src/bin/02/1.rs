use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).expect("Failed to read line");
    let result: u32 = buffer
        .trim()
        .split("\n")
        .map(|line| {
            line.split("x")
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|t| {
            let a = t[0];
            let b = t[1];
            let c = t[2];
            
            let areas = vec![a*b, a*c, b*c];

            let area = 2*(areas.iter().sum::<u32>());
            let extra = areas.iter().min().unwrap();
            area + extra
        })
        .sum();
    println!("{}", result);
}
