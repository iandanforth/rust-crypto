#[derive(Debug)] // This allows us to use automatic debug formatting on instances of this struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(width1, height1)
    );

    let rect1 = Rectangle { width: 30, height: 40};

    println!("{:?}", rect1); // Basic debug output
    println!("{:#?}", rect1); // Multi line debug output (I think)

    let rect1_area = area_struct(rect1);

    println!("The area of the struct-rect is {}", rect1_area);

}

fn area(width: u32, height: u32) -> u32 {
    let area = width * height;
    return area;
}

fn area_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}