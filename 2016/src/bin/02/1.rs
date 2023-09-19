use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let mut x = -2;
    let mut y = 1;
    for line in buffer.trim().split("\n") {
        for char in line.chars() {
            match char {
                'U' => if y > 0 { y -= 1},
                'D' => if y < 2 { y += 1},
                'L' => if x > 0 { x -= 1},
                'R' => if x < 2 { x += 1},
                _ => {}
            }
        }
        print!("{}", 1 + x + 3*y)
    }
    println!();
}
