use std::{
    collections::HashSet,
    io::{self, Read},
    process::exit,
};

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let mut result = 0;
    let mut tmp = HashSet::new();
    loop {
        for delta in buffer.trim().split("\n") {
            let delta_num = delta.parse::<i32>().unwrap();
            result += delta_num;
            if tmp.contains(&result) {
                println!("{}", result);
                exit(0);
            }
            tmp.insert(result);
        }
    }
}
