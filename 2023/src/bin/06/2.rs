use std::io;
use std::io::Read;

fn parse(lines: &mut std::str::Split<'_, char>) -> (u64, u64) {
    let time_str = lines.next().unwrap();
    let times: Vec<_> = time_str[5..]
        .trim()
        .split(' ')
        .filter(|&part| !part.is_empty())
        .collect();
    
    let time = times.join("").parse::<u64>().unwrap();

    let dist_str = lines.next().unwrap();
    let distances: Vec<_> = dist_str[9..]
        .trim()
        .split(' ')
        .filter(|&part| !part.is_empty()).collect();
    let distance = distances.join("").parse::<u64>().unwrap();
    (time, distance)
}
fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let mut lines = buffer.trim().split('\n');
    let (times, distances) = parse(&mut lines);
    let numbers = vec![(times, distances)]
        .iter()
        .map(|(t, d)| {
            let discriminant_sq = (t * t - 4 * d) as f64;
            let discriminant = discriminant_sq.sqrt() - 1e-3;
            let start = (((*t as f64) - discriminant) / 2.0).ceil() as u64;
            let end = (((*t as f64) + discriminant) / 2.0).floor() as u64;
            end - start + 1
        })
        .reduce(|accum, elem| accum * elem)
        .unwrap();
    println!("{:?}", numbers);
}
