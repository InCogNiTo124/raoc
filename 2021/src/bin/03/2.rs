use std::cmp::Ordering;
use std::io::{self, Read};

fn filter_min(mut numbers: Vec<&str>) -> u32 {
    let mut i = 0;
    while numbers.len() > 1 {
        let n = numbers.len();
        let ones = numbers
            .iter()
            .filter(|&line| line.get(i..(i+1)).unwrap() == "1")
            .count();
        let key = match ones.cmp(&(n - ones)) {
            Ordering::Less => "1",
            Ordering::Equal => "0",
            Ordering::Greater => "0",
        };
        numbers = numbers
            .iter()
            .filter(|&&line| line.get(i..(i+1)).unwrap() == key)
            .map(|v| *v)
            .collect();
        i += 1;
    }
    u32::from_str_radix(numbers[0], 2).unwrap()
}

fn filter_max(mut numbers: Vec<&str>) -> u32 {
    let mut i = 0;
    while numbers.len() > 1 {
        let n = numbers.len();
        let ones = numbers
            .iter()
            .filter(|&line| line.get(i..(i+1)).unwrap() == "1")
            .count();
        let key = match ones.cmp(&(n - ones)) {
            Ordering::Less => "0",
            Ordering::Equal => "1",
            Ordering::Greater => "1",
        };
        numbers = numbers
            .iter()
            .filter(|&&line| line.get(i..(i+1)).unwrap() == key)
            .map(|v| *v)
            .collect();
        i += 1;
    }
    u32::from_str_radix(numbers[0], 2).unwrap()
}

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_to_string(&mut buffer);
    let numbers = buffer.trim().split_whitespace().collect::<Vec<_>>();
    let o2 = filter_max(numbers.clone());
    let co2 = filter_min(numbers.clone());
    println!("{}, {}, {}", o2, co2, o2 * co2);
    //println!("{}", co2);
}
