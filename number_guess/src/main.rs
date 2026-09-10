use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to the Guessing game!");

    let answer = rand::rng().random_range(1..=100);

    loop {
        println!("Guess a number and enter it here:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to receive the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid type");
                continue;
            }
        };

        match guess.cmp(&answer) {
            Ordering::Less => println!("Your number is too small"),
            Ordering::Greater => println!("Your number is too big"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
