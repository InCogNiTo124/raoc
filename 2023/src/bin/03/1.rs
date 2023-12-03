use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let re_nums = Regex::new(r"\d+").unwrap();
    let re_syms = Regex::new(r"[^\.\d]").unwrap();
    let mut digit_positions = Vec::new();
    let mut symbol_positions = HashMap::new();
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().split('\n').enumerate().for_each(|(i, row)| {
        for captures in re_nums.captures_iter(row) {
            if let Some(capture) = captures.get(0) {
                let number = capture.as_str().parse::<u32>().unwrap();
                digit_positions.push((number, i, capture.range()));
            }
        }
        for captures in re_syms.captures_iter(row) {
            if let Some(capture) = captures.get(0) {
                // dbg!(&capture);
                // panic!();
                for j in capture.range() {
                    symbol_positions.insert((i, j), capture.as_str());
                }
            }
        }
    });
    let result = digit_positions
        .iter()
        .filter_map(|(number, row, range)| {
            // let (i, range) = range;
            for y in row.saturating_sub(1)..=(row + 1) {
                for x in range.start.saturating_sub(1)..(range.end + 1) {
                    if symbol_positions.contains_key(&(y, x)) {
                        return Some(number);
                    }
                }
            }
            None
        })
        // .reduce(|accum, elem| accum * elem)
        .sum::<u32>();
    // .unwrap();
    println!("{}", result);
}
