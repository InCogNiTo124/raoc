use std::io;

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
    let mut result_x = 0;
    let mut result_y = 0;
    let mut heading = Heading::North;
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
        match heading {
            Heading::North => result_y += value,
            Heading::West => result_x -= value,
            Heading::South => result_y -= value,
            Heading::East => result_x += value,
        }
    }
    println!("{}", result_x.abs() + result_y.abs())
}
