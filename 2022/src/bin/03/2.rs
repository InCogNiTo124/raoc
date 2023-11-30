use std::{
    collections::HashSet,
    io::{self, Read},
};
fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let lines = buffer.trim().split("\n").collect::<Vec<_>>();
    let mut values: Vec<u32> = Vec::new();
    for i in (0..lines.len()).step_by(3) {
        let s1 = &lines[i].chars().collect::<HashSet<_>>();
        let s2 = &lines[i + 1].chars().collect::<HashSet<_>>();
        let s3 = &lines[i + 2].chars().collect::<HashSet<_>>();
        let i1 = &(s1 & s2);
        let i2 = &(i1 & s3);
        let inter = i2.iter().collect::<Vec<_>>();
        
        let c = *inter[0];
        let value = if c.is_uppercase() {
            (c as u32) - 64 + 26
        } else {
            (c as u32) - 96
        };
        values.push(value);
    }
    let result = values.iter().sum::<u32>();
    println!("{}", result);
}
