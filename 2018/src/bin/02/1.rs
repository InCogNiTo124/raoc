use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let result = buffer
        .trim()
        .split("\n")
        .map(|s| {
            let mut counts = HashMap::<char, u32>::new();
            for c in s.chars() {
                if counts.contains_key(&c) {
                    *counts.get_mut(&c).unwrap() += 1;
                } else {
                    counts.insert(c.clone(), 1);
                }
            }
            let values = counts.values().collect::<HashSet<&u32>>();
            let doubles = values.contains(&2);
            let triples = values.contains(&3);
            (doubles as u32, triples as u32)
        })
        .fold((0, 0), |accum, elem| (accum.0+elem.0, accum.1 + elem.1));
    println!("{}", result.0 * result.1);
}
