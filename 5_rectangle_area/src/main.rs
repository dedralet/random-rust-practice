struct Rectangle {
    height: u32,
    width: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    let rectangle = Rectangle {
        height: 40,
        width: 20,
    };

    println!(
        "Area of the rectangle is equal to {}",
        area(&rectangle)
    );
}
