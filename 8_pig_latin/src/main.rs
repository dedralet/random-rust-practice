use std::io;

fn translation(input: String) -> String {
    let mut felina: String = String::new();

    for word in input.split_whitespace() {
        let temp = word.chars();
        match &temp.as_str()[0..1] {
            "a" | "e" | "i" | "o" | "u" => felina.push_str(&(format!("{word}-hay"))),
            _ => felina.push_str(&(format!("{&temp[1..temp.len()]}-{&temp[0..1]}ay"))),
        }
        felina.push(' ');
    }

    felina
}

fn main() {
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to receive the input. Exiting...");
    
    let felina: String = translation(input);

    println!("Here is Pig Latin variation of your text: {}", felina);
}
