use regex::Regex;
use std::io::{self, Read};

fn main() {
    let re_nums = Regex::new(r"\d+").unwrap();
    let mut row_numbers = Vec::new();
    let mut mul_places = Vec::new();
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().split('\n').enumerate().for_each(|(i, row)| {
        let mut numbers = Vec::new();
        for captures in re_nums.captures_iter(row) {
            if let Some(capture) = captures.get(0) {
                let number = capture.as_str().parse::<u32>().unwrap();
                let y_range = i.saturating_sub(1)..=i.saturating_add(1);
                let x_range = capture.start().saturating_sub(1)..capture.end().saturating_add(1);
                numbers.push((number, (y_range, x_range)));
            }
        }
        row_numbers.push(numbers);

        row.match_indices('*').for_each(|(index, _asterisk)| {
            mul_places.push((i, index));
        })
    });
    // dbg!(&mul_places);
    let result = mul_places
        .iter()
        .filter_map(|(i, j)| {
            let mut numbers_near_mul = Vec::<u32>::new();

            // row above '*'
            row_numbers
                .get(i - 1)
                .unwrap()
                .iter()
                .for_each(|(value, (range_i, range_j))| {
                    if range_i.contains(i) && range_j.contains(j) {
                        numbers_near_mul.push(*value)
                    }
                });

            // row of '*'
            row_numbers
                .get(*i)
                .unwrap()
                .iter()
                .for_each(|(value, (range_i, range_j))| {
                    if range_i.contains(i) && range_j.contains(j) {
                        numbers_near_mul.push(*value)
                    }
                });

            // row below '*'
            row_numbers
                .get(i + 1)
                .unwrap()
                .iter()
                .for_each(|(value, (range_i, range_j))| {
                    if range_i.contains(i) && range_j.contains(j) {
                        numbers_near_mul.push(*value)
                    }
                });

            // dbg!(&numbers_near_mul);
            if numbers_near_mul.len() == 2 {
                Some(numbers_near_mul.get(0).unwrap() * numbers_near_mul.get(1).unwrap())
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("{}", result);
}
