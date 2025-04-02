#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// impl PartialEq for Rectangle {
//     fn eq(&self, other: &Self) -> bool {
//         self.width == other.width && self.height == other.height
//     }
// }

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// pub fn add_g<T: Add<Output = T> + Copy>(left: T, right: T) -> T {
//     left + right
// }

pub fn greeting(name: &str) -> String {
    let greet = format!("Hello! {name}");
    String::from(greet)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 3,
            height: 2,
        };

        let larger = Rectangle {
            width: 5,
            height: 8,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn two_rectangle_are_equal() {
        let rect_1 = Rectangle {
            width: 5,
            height: 5,
        };
        let rect_2 = Rectangle {
            width: 5,
            height: 5,
        };

        assert_eq!(rect_1, rect_2);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    #[test]
    #[should_panic]
    // #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // #[test]
    // fn add_integers() {
    //     let result = add_g(1, 2);
    //     assert_eq!(result, 3);
    // }

    // #[test]
    // fn add_floats() {
    //     let result = add_g(2.5, 2.5);
    //     assert_eq!(result, 5.0);
    // }
}
