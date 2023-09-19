use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer).expect("Failed to read line"); 
    let mut result = 0;
    for char in buffer.chars() {
        match char {
            '(' => {result += 1;},  
            ')' => {result -= 1;},
            _ => {}
        }
    }
    println!("{}", result);
}
