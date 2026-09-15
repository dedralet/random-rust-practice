use std::io;

fn main() {
    println!("Welcome to Fibonacci number generator!");

    loop {
        println!("Enter a number:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to receive the input");

        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again");
                continue;
            }
        };

        let mut f_a = 1;
        let mut f_b = 1;

        if n == 0 {
            println!("F(0) is equal to 0");
        } else if n <= 2 {
            println!("F(1) is equal to 1");
        } else {
            let mut f_n = f_a + f_b;

            for _ in 3..=n {
                f_n = f_a + f_b;
                f_a = f_b;
                f_b = f_n;
            }

            println!("F({n}) is equal to {f_n}");
        }

        break;
    }
}
