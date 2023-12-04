use scanf::sscanf;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let result = buffer.trim().split('\n').map(|line|{
        let mut card_id: u32 = 0;
        let mut winning: String = String::new();
        let mut card: String = String::new();
        match sscanf!(line, "Card {}: {} | {}", card_id, winning, card) {
            Ok(_) => {},
            Err(x) => panic!("{}", x)
        };
        let win_num: HashSet<_> = winning.split(' ').filter_map(|maybe_number| str::parse::<u32>(maybe_number).ok()).collect();
        let card_num: HashSet<_> = card.split(' ').filter_map(|maybe_number| str::parse::<u32>(maybe_number).ok()).collect();
        // dbg!(&win_num, &card_num);
        let count = win_num.intersection(&card_num).count();
        match count {
            0 => 0,
            _ => 2_u32.pow((count-1).try_into().unwrap())
        }
    }).sum::<u32>();
    println!("{}", result);
}
