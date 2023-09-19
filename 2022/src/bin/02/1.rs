use std::io::{self, Read};
fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let result = buffer
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.split(' ');
            let opponent = (parts.next().unwrap().chars().next().unwrap() as i32) - 65;
            let me = (parts.next().unwrap().chars().next().unwrap()) as i32 - 88;
            // println!("{} {}", opponent, me);

            let score = match (me - opponent).rem_euclid(3) {
                0 => 3,
                1 => 6,
                2 => 0,
                _ => panic!("Error in math itself"),
            };
            score + me + 1
        })
        .sum::<i32>();
    println!("{}", result);
}
