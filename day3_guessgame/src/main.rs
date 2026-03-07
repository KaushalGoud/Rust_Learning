// Guessing Game
use std::io;
fn main() {
    println!("Guess the number!");
    let random_number = 10;
    let mut input: String = String::new();
    loop {
        println!("What's your guess?");
        io::stdin().read_line(&mut input);
        let mut guess_number = input.trim().parse().unwrap();
        if random_number == guess_number {
            println!("Correct Guess!!! ie.{}",guess_number);
            break;
        }
        else {
            input.clear();
            println!("😭Try Again!!");
        }
    }
}
