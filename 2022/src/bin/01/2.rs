use std::cmp::Reverse;
use std::collections::BinaryHeap;
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
    vectors.push(tmp);

    let sizes = vectors.iter().map(|v| {
        v.iter().sum::<i32>()});
    let mut heap = BinaryHeap::new();
    for item in sizes {
        heap.push(Reverse(item));
        if heap.len() > 3 {
            let _ = heap.pop();
        } 
    }
    let result : i32= heap.iter().map(|rev| rev.0).sum();
    println!("{}", result);
}
