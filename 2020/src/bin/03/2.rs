use std::io::{self, Read};

fn trees_for_direction(field: &Vec<&str>, dx: i32, dy: i32) -> i64 {
    let mut count = 0;
    let mut x: i32 = 0;
    for i in (0..field.len()).step_by(dy.try_into().unwrap()) {
        let row = &field[i];
        if row.chars().nth(x.try_into().unwrap()).unwrap() == '#' {
            count += 1;
        }
        x = (x + dx).rem_euclid((row.len() as i32).try_into().unwrap());
    }
    count
}

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let field = buffer.trim().split('\n').collect::<Vec<_>>();
    let result = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| trees_for_direction(&field, *dx, *dy))
        .reduce(|accum, elem| accum * elem)
        .unwrap();
    println!("{}", result);
}
