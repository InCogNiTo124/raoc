use std::io;

fn main() {
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer).unwrap();
    let mut sum = 0;
    let length = buffer.len()-1;  // avoiding newline
    for i in 0..length {
        let letter = buffer.chars().nth(i).unwrap();
        let next_letter = buffer.chars().nth((i+length/2)%length).unwrap();
        if letter == next_letter {
            sum += letter.to_digit(10).unwrap();
        }
    }
    println!("{}", sum);
}