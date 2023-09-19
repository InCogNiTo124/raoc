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
            let target = (parts.next().unwrap().chars().next().unwrap()) as i32 - 89;
            // println!("{} {}", opponent, me);
            let me = (opponent + target).rem_euclid(3);

            let score = match target {
                0 => 3,
                1 => 6,
                -1 => 0,
                _ => panic!("Error in math itself"),
            };
            score + me + 1
        })
        .sum::<i32>();
    println!("{}", result);
}
