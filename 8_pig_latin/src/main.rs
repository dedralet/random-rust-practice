use std::io;

fn translation(input: String) -> String {
    let mut felina: String = String::new();

    for word in input.split_whitespace() {
        match &word.chars().next() {
            Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => {
                felina.push_str(&(format!("{word}-hay")))
            }
            _ => {
                let temp = &word[1..];
                for letter in temp.chars() {
                    felina.push(letter);
                }
                felina.push('-');
                felina.push(word.chars().next().unwrap());
                felina.push_str("ay");
            }
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
