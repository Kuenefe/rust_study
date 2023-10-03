#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self: &Self) -> u32 { //&self
        self.width * self.height
    }

    fn width(self: &Self) -> bool {
        self.width > 0
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn height(self: &Self) -> bool {
        self.height > 0
    }

    fn can_hold(self: &Self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {

    let rect: Rectangle = Rectangle { width: 30, height: 50 };
    let rect2: Rectangle = Rectangle { width: 20, height: 40 };

    let rect3: Rectangle = Rectangle::square(66);

    let size: bool = rect.can_hold(&rect2);

    println!("method result: {}", size);
}

