#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// associated functions with Rectabnle
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 45,
    };

    let rect3 = Rectangle {
        width: 34,
        height: 23,
    };

    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1hold rect3? {}", rect1.can_hold(&rect3));
}
