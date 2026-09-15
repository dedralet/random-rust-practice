use std::io;

fn main() {
    println!("Celsius to Fahrenheit converter");

    loop {
        println!("Enter a value:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Input is not received");
        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Try again");
                continue;
            }
        };
        let fahrenheit: f64 = input * 9. / 5. + 32.0;
        println!("{input} °C = {fahrenheit} °F");
        break;
    }
}
