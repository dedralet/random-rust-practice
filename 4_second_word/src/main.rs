use std::io;

fn second_word(s: &str) -> &str {
    let stringus = s.as_bytes();
    let mut first_space = 0;
    let mut started = false;

    for (i, &iter) in stringus.iter().enumerate() {
        if started == false && iter != b' ' {
            started = true;
        } else if started == true {
            if iter == b' ' {
                if first_space == 0 {
                    first_space = i + 1;
                } else {
                    return &s[first_space..i];
                }
            }
        }
    }
    "Input contains only one word!"
}

fn main() {
    println!("Enter your input:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let second_word = second_word(&input);

    println!("{second_word}");
}
