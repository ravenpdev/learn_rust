#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    {
        // Defining Methods
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!("The area of the rectangle is {} square pixels", rect.area());

        if rect.width() {
            println!("The rectangle has a nonzero width; it is {}", rect.width);
        }

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };

        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("can rect hold rect2? {}", rect.can_hold(&rect2));
        println!("can rect hold rect3? {}", rect.can_hold(&rect3));

        let square = Rectangle::square(30);
        dbg!(&square);
    }
}
