use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let mut x =0;
    let mut y = 0;
    let mut positions = HashSet::<(i32, i32)>::new();
    positions.insert((x, y));
    buffer.trim().chars().for_each(|c| {
        match c {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => {}
        }
        positions.insert((x, y));
    });
    println!("{}", positions.len());
}
