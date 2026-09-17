fn area_of_rectangle(lengths: (u32, u32)) -> u32 {
    lengths.0 * lengths.1
}

fn main() {
    let rectangle: (u32, u32) = (40, 20);

    println!(
        "Area of the rectangle is equal to {}",
        area_of_rectangle(rectangle)
    );
}
