use std::io;

fn first_word(s: &str) -> &str {
    let byte = s.as_bytes();

    for (i, &car) in byte.iter().enumerate() {
        if car == b' ' 
        {
            return &s[..i];
        };
    }
    &s[..]
}

fn main(){
    let mut stringus = String::new();
    
    io::stdin()
        .read_line(&mut stringus)
        .expect("Failed to read input");

    let s = &stringus;
    println!("{}", first_word(s));
}
