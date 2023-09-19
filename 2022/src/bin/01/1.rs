use std::io::{self, Read};
fn main() {
    let mut vectors: Vec<Vec<i32>> = Vec::new();
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let mut tmp: Vec<i32> = Vec::new();
    for line in buffer.trim().split("\n") {
        if line == "" {
            vectors.push(tmp.clone());
            tmp.clear();
            continue;
        }
        let value = line.parse::<i32>().unwrap();
        tmp.push(value);
    }

    let result = vectors.iter().map(|v| {
        /*println!("{:?}", v);*/
        v.iter().sum::<i32>()}).max();
    println!("{}", result.unwrap());
}
