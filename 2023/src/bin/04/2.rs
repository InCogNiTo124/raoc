use scanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::Read;

fn main() {
    let mut copies: HashMap<usize, u32> = HashMap::new();
    copies.insert(0, 1);
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    buffer.trim().split('\n').enumerate().for_each(|(i, line)| {
        if !copies.contains_key(&i) {
            copies.insert(i, 1);
        }
        let mut card_id: u32 = 0;
        let mut winning: String = String::new();
        let mut card: String = String::new();
        match sscanf!(line, "Card {}: {} | {}", card_id, winning, card) {
            Ok(_) => {}
            Err(x) => panic!("{}", x),
        };
        let win_num: HashSet<_> = winning
            .split(' ')
            .filter_map(|maybe_number| str::parse::<u32>(maybe_number).ok())
            .collect();
        let card_num: HashSet<_> = card
            .split(' ')
            .filter_map(|maybe_number| str::parse::<u32>(maybe_number).ok())
            .collect();
        // dbg!(&win_num, &card_num);
        let count = win_num.intersection(&card_num).count();
        // dbg!(&count);
        if count > 0 {
            // print('\t');
            let this_count = *copies.get(&i).unwrap();
            // dbg!(this_count);
            for j in i..(i + count) {
                let index = j+1;
                // dbg!(copies.get(&index));
                if !copies.contains_key(&index) {
                    copies.insert(index, 1);
                }
                *copies.get_mut(&index).unwrap() += this_count;
                // dbg!(copies.get(&index));
            }
        }
        // dbg!(copies.values().sum::<u32>());
    });
    // dbg!(copies);
    println!("{}", copies.values().sum::<u32>());
}
