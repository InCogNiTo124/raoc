use std::io;
use std::io::Read;
use std::ops::Range;

#[derive(Debug)]
struct Map {
    ranges: Vec<(Range<u64>, i64)>,
}

impl Map {
    fn new() -> Self {
        Self { ranges: vec![] }
    }
    fn add_range(&mut self, dst_range_start: u64, src_range_start: u64, range_len: u64) {
        let range = src_range_start..(src_range_start + range_len);
        let delta = (dst_range_start as i64) - (src_range_start as i64);
        self.ranges.push((range, delta.try_into().unwrap()));
        // todo optimize
        // if this well ordered on src_range_start, I could use binary search
    }

    fn map(&self, value: u64) -> u64 {
        for (range, delta) in self.ranges.iter() {
            if range.contains(&value) {
                return value.wrapping_add_signed(*delta);
            }
        }
        value
    }
}
fn parse_map(lines: &mut std::str::Split<'_, char>) -> Option<Map> {
    let mut map = Map::new();

    // skip header
    match lines.next() {
        None => return None, // end of lines
        Some(x) => {}        // ignore - empty line
    }
    lines.take_while(|&line| !line.is_empty()).for_each(|line| {
        let numbers: Vec<_> = line
            .split(' ')
            .filter_map(|t| t.parse::<u64>().ok())
            .collect();
        assert_eq!(numbers.len(), 3);
        map.add_range(numbers[0], numbers[1], numbers[2]);
    });
    Some(map)
}
fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");

    let mut lines = buffer.trim().split('\n');
    let seeds = lines.next().unwrap();
    let seeds = seeds[7..]
        .split(' ')
        .filter_map(|t| t.parse::<u64>().ok())
        .collect::<Vec<_>>();
    let _ = lines.next(); // skip empty line
    // dbg!(&seeds);

    // elegant input parsing 😎
    let mut map_vec: Vec<Map> = Vec::new();
    while let Some(map) = parse_map(&mut lines) {
        map_vec.push(map);
    }
    // dbg!(&map_vec);

    let result = seeds.iter().map(|seed| {
        let mut value = *seed;
        for map in map_vec.iter() {
            value = map.map(value);
        }
        value
    }).min().unwrap();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn test_map() {
        let mut map = Map::new();
        map.add_range(50, 98, 2);
        map.add_range(52, 50, 48);

        assert_eq!(map.map(0), 0);
        assert_eq!(map.map(51), 53);
        assert_eq!(map.map(98), 50);
    }
}
