fn area_of_rectangle(height: u32, width: u32) -> u32 {
    height * width
}

fn main() {
    let height = 40;
    let width = 20;

    println!(
        "Area of the rectangle is equal to {}",
        area_of_rectangle(height, width)
    );
}
