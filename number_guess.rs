use std::io;
use std::cmp::Ordering;
use rand::RngExt;

fn main(){
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Guess a number and enter it here: ");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid type. Try again");
                    continue;
                },
            };

        println!("your number is {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less    => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal   => {
                println!("you won!");
                break;
            },
        }
    }
}
