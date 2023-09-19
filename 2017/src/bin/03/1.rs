use std::f32::consts::FRAC_PI_2;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let value = buffer.trim().parse::<u32>().unwrap();
    let result = (2..=value)
        .map(|n| {
            let x: f32 = (n as f32) * 4.0 - 7.0;
            let t = x.sqrt().floor() * FRAC_PI_2;
            let (s, c) = t.sin_cos();
            (s as i32, c as i32)
        })
        .reduce(|(a, b), (c, d)| (a + c, b + d))
        .unwrap();
    println!("{}", result.0.abs() + result.1.abs());
}
