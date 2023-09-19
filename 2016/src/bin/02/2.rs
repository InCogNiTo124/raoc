use std::{
    io::{self, Read},
    vec,
};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let keypad = vec![
        vec!['1'],
        vec!['2', '3', '4'],
        vec!['5', '6', '7', '8', '9'],
        vec!['A', 'B', 'C'],
        vec!['D'],
    ];
    let mut x: i32 = -2;
    let mut y: i32 = 0;
    for line in buffer.trim().split("\n") {
        for char in line.chars() {
            match char {
                'U' => {
                    if 2 - x > y && 2 + x > y {
                        y += 1
                    }
                }
                'D' => {
                    if x - 2 < y && -2 - x < y {
                        y -= 1
                    }
                }
                'L' => {
                    if y < 2 + x && y > -2 - x {
                        x -= 1
                    }
                }
                'R' => {
                    if y < 2 - x && y > -2 + x {
                        x += 1
                    }
                }
                _ => {}
            }
            assert!(x.abs() + y.abs() <= 2);
        }
        print!(
            "{}",
            keypad[(2-y) as usize][(x + 2 - (y as i32).abs()) as usize]
        )
    }
    println!();
}
