use std::{
    collections::HashSet,
    io::{self, Read},
};
fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let result = buffer
        .trim()
        .split("\n")
        .map(|line| {
            let n = line.len() / 2;
            (&line[0..n], &line[n..])
        })
        .map(|(s1, s2)| {
            let chars1 = s1.chars().collect::<HashSet<_>>();
            let chars2 = s2.chars().collect::<HashSet<_>>();
            let intersection = chars1.intersection(&chars2).collect::<Vec<_>>();
            *intersection[0]
        })
        .map(|c| {
            let value = if c.is_uppercase() {
                (c as u32) - 64 + 26
            } else {
                (c as u32) - 96
            };
            value
        })
        .sum::<u32>();
    println!("{}", result);
}
