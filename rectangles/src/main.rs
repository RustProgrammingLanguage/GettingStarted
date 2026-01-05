// // Listing 5-8: Calculating the area of a rectangle specified by separate width and height variables
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// // Listing 5-9: Calculating the area of a rectangle specified by a tuple
// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Listing 5-10: Calculating the area of a rectangle specified by a struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:#?}");

    println!(
        "\nThe area of the rectangle is {} square pixels.",
        area(&rect1) // we want to borrow the struct rather than take ownership of it
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height // does not move the field values
}