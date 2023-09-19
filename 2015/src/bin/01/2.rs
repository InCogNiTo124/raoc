use std::io;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut buffer).expect("Failed to read line"); 
    let mut result = 0;
    for (i, char) in buffer.chars().enumerate() {
        match char {
            '(' => {result += 1;},  
            ')' => {result -= 1;},
            _ => {}
        }
        if result == -1 {
            println!("{}", i+1);
            break;
        }
    }
}
