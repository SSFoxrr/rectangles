//area of rectangle - struct practice

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} sq pixels",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// refactoring with tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {}",
//         area2(rect1)
//     );
// }

// fn area2(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// refactoring with structs

struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "the area of the rectangle is {}",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//using derived traits for debugging

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:#?}");
// }

// creating methods

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {}",
//         rect1.area()
//     );
// }

// creating methods with more parameters
// program to determine if a rectangle can fit in another
// rectangle

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can it fit? {}", rect1.can_hold(&rect2));
    println!("Can it fit? {}", rect1.can_hold(&rect3));
}