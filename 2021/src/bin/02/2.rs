use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let mut position: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;
    for line in buffer.trim().split('\n') {
        let mut parts = line.split(' ');
        let string = parts.next().unwrap();
        let number = parts.next().unwrap().parse::<u32>().unwrap();
        match string {
            "forward" => {
                position += number;
                depth += number * aim;
            }
            "down" => aim += number,
            "up" => aim -= number,
            _ => { /* error */ }
        }
    }
    println!("{}", position * depth);
}
