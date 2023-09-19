use std::collections::HashMap;
use std::f32::consts::FRAC_PI_2;
use std::io::{self, Read};

fn coordinates(n: &f32) -> (i32, i32) {
    let x: f32 = *n * 4.0 - 7.0;
    let t = x.sqrt().floor() * FRAC_PI_2;
    let (s, c) = t.sin_cos();
    (s as i32, c as i32)
}

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let value = buffer.trim().parse::<u32>().unwrap();
    let mut field : HashMap<(i32, i32), u32> = HashMap::new();
    field.insert((0, 0), 1);
    let mut coords = (0, 0);
    for n in 2.. {
        let new_coords = coordinates(&(n as f32));
        coords.0 += new_coords.0;
        coords.1 += new_coords.1;
        let (x, y) = coords;
        let mut value_at_field = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if !(dx == 0 && dy == 0) {
                    let neighbour = match field.get(&(x+dx, y+dy)) {
                        None => 0,
                        Some(n) => *n
                    };
                    value_at_field += neighbour;
                }
            }
        }
        if value_at_field > value {
            println!("{}", value_at_field);
            break;
        }
        field.insert(coords, value_at_field);
    }
}
