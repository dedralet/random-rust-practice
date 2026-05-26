use std::io;

fn main(){
    let mut f_1: u16 = 0;
    let mut f_2: u16 = 1;
    loop {
        println!("Enter a natural number: ");

        let mut n = String::new();
        
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read input.");

        let n: u16 = match n
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_)  => {
                    println!("Enter a natural number");
                    continue;
                }
            };

        if n == 0 {
            println!("The f(0) = {f_1}");
            break;
        }
        else if n == 1 {
            println!("The f(1) = {f_2}");
            break;
        }
        
        let mut f_n = 0;
        let mut count: u16 = n - 1;
        while count > 0 {
            f_n = f_1 + f_2;
            f_1 = f_2;
            f_2 = f_n;
            count -= 1;
        }
        println!("The f({n}) = {f_n}");
        break;
    }
}
