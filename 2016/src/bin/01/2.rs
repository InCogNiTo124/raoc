use std::collections::HashSet;
use std::io;
use std::process::exit;

enum Heading {
    North,
    East,
    South,
    West,
}

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer).expect("Failed to read line");
    let mut result_x: i32 = 0;
    let mut result_y: i32 = 0;
    let mut heading = Heading::North;
    let mut set = HashSet::new();
    set.insert((0, 0));
    for char in buffer.split(", ") {
        if char.starts_with("L") {
            heading = match heading {
                Heading::North => Heading::West,
                Heading::West => Heading::South,
                Heading::South => Heading::East,
                Heading::East => Heading::North,
            };
        } else if char.starts_with("R") {
            heading = match heading {
                Heading::North => Heading::East,
                Heading::East => Heading::South,
                Heading::South => Heading::West,
                Heading::West => Heading::North,
            };
        }
        let value = char[1..].parse::<i32>().expect("Expected a number");
        for _ in 1..=value {
            match heading {
                Heading::North => {
                    result_y += 1;
                }
                Heading::West => {
                    result_x -= 1;
                }
                Heading::South => {
                    result_y -= 1;
                }
                Heading::East => {
                    result_x += 1;
                }
            }
            if set.contains(&(result_x, result_y)) {
                println!("{}", result_x.abs() + result_y.abs());
                exit(0);
            }
            set.insert((result_x, result_y));
        }
    }
}
