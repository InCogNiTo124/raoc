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
    let mut positions = vec![(0, 0), (0, 0)];
    let mut past_positions = HashSet::<(i32, i32)>::new();
    past_positions.insert((0, 0));

    buffer.trim().chars().enumerate().for_each(|(index, c)| {
        let mut pos = positions.get_mut(index%2).unwrap();
        match c {
            '^' => pos.1 += 1,
            '>' => pos.0 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            _ => {}
        }
        past_positions.insert(pos.clone());
    });
    println!("{}", past_positions.len());
}
