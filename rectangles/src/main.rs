#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect_inside: &Rectangle) -> bool {
        self.width >= rect_inside.width && self.height >= rect_inside.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let scale = 2;

    let rectangle1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };

    println!("rectangle1: {rectangle1:?}");

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );

    dbg!(&rectangle1);

    let square1 = Rectangle::square(30);

    dbg!(&square1);
}
