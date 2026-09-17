struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn is_fit(&self, other: &Rectangle) -> bool {
        if (self.width >= other.width && self.height >= other.height)
            || (self.width >= other.height && self.height >= other.width)
        {
            return true;
        }
        false
    }
}

fn main() {
    let rectangle1 = Rectangle {
        height: 40,
        width: 20,
    };

    let rectangle2 = Rectangle {
        height: 10,
        width: 40,
    };

    let rectangle3 = Rectangle {
        height: 50,
        width: 20,
    };

    println!(
        "Rectangle2 fits into Rectangle1: {}",
        rectangle1.is_fit(&rectangle2)
    );
    println!(
        "Rectangle3 fits into Rectangle1: {}",
        rectangle1.is_fit(&rectangle3)
    );
}
