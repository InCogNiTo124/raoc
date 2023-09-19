use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let mut position: u32 = 0;
    let mut depth: u32 = 0;
    for line in buffer.trim().split('\n') {
        let mut parts = line.split(' ');
        match parts.next().unwrap() {
            "forward" => position += parts.next().unwrap().parse::<u32>().unwrap(),
            "down" => depth += parts.next().unwrap().parse::<u32>().unwrap(),
            "up" => depth -= parts.next().unwrap().parse::<u32>().unwrap(),
            _ => { /* error */ }
        }
    }
    println!("{}", position * depth);
}
