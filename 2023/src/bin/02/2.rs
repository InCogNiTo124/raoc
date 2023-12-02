use std::io::{self, Read};
fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    let _ = stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read line");
    let result = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut parts = line.split(':');
            let game_id = parts.next().unwrap()[5..].parse::<i32>().unwrap();
            let draws = parts.next().unwrap().trim();
            let max_cubes = draws
                .split("; ")
                .map(|cubes| {
                    let mut cube_counts = (0, 0, 0);
                    for cube in cubes.split(", ") {
                        let mut parts = cube.split(' ');
                        let count = parts.next().unwrap().parse::<i32>().unwrap();
                        let color = parts.next().unwrap();
                        match color {
                            "red" => cube_counts.0 = count,
                            "green" => cube_counts.1 = count,
                            "blue" => cube_counts.2 = count,
                            _ => panic!("wrong color"),
                        };
                    }
                    cube_counts
                })
                .reduce(|accum, elem| {
                    (
                        accum.0.max(elem.0),
                        accum.1.max(elem.1),
                        accum.2.max(elem.2),
                    )
                }).unwrap();
            (max_cubes.0*max_cubes.1*max_cubes.2)
        })
        .sum::<i32>();
    println!("{}", result);
}
