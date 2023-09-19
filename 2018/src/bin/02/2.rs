use std::{
    io::{self, Read},
    process::exit,
};

fn index_of_difference(a: &str, b: &str) -> Option<usize> {
    // a and b are assumed to have equal lengths
    let mut indices = Vec::new();
    for i in 0..a.len() {
        if a.chars().nth(i) != b.chars().nth(i) {
            indices.push(i);
            if indices.len() > 1 {
                return None;
            }
        }
    }
    Some(indices[0])
}

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let lines: Vec<&str> = buffer.trim().split("\n").collect();
    for i in 0..(lines.len() - 1) {
        let a = lines[i];
        for j in (i + 1)..lines.len() {
            let b = lines[j];
            match index_of_difference(a, b) {
                Some(t) => {
                    for i in 0..a.len() {
                        if i != t {
                            print!("{}", a.chars().nth(i).unwrap());
                        }
                    }
                    println!();
                    exit(0);
                }
                None => {}
            }
        }
    }
}
