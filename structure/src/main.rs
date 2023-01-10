//Find Area of Rectangle

//1 way of doing
// fn main() {
//        println!("1 way of doing:-");
//     let width1 = 10;
//     let height1 = 20;

//     println!("The area of rectangle is {} square pixels.",area(width1,height1));
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

//2 way of doing by using tuple type
// fn main() {
//     println!("2 way of doing by using tuple type:-");
//     let rect1 = (20, 30);
//     println!("The area of rectangle is {} square pixels.",area(rect1));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

////3 way of doing by using structure
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// fn main() {
//     println!("3 way of doing by using structure:-");
//     let rect1 = Rectangle {
//         width: 30,
//         height: 40,
//     };
//     println!("The area of the rectangle is {} square pixels.",area(&rect1));
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//printing the structure
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
    // dbg!(&rect1);
}