struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rectangle = Rectangle {
        height: 40,
        width: 20,
    };

    println!("Area of the rectangle is equal to {}", rectangle.area());
}
