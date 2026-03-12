use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number between 1 to 10!");

    let random_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut input = String::new();

        print!("What's your guess? huh->");

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let input: i32 = input.trim().parse().expect("Please type a number!");

        println!("Your Guess: {}", input);

        if input == random_number {
            println!("Correct Guess!!! ie. {}", input);
            break;
        } else {

            print!("😭");
            println!(" Try Again!! ");
            print!("Correct Answer :{random_number}\n");
        }

        match input.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You nailed it");
                break;
            }
        }
    }
}
