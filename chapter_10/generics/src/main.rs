fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 80, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = [1, 45, 10, 77];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = ['a', 'e', 'i', 'o', 'u'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    // Generic in Struct definition
    {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        println!("{integer:#?}");
        println!("{float:#?}");
    }

    // Generic in Struct with different types
    {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let mix = Point { x: 5, y: 10.5 };
        println!("{mix:#?}");
    }

    // In Enum Definitions
    {
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    // In Method Definitions
    {
        struct Point<T> {
            x: T,
            y: T,
            z: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        // Only applies on f32 type
        impl Point<f64> {
            fn distance_from_origin(&self) -> f64 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        // impl<T> Point<T, char> {
        //     fn z(&self) -> &char {
        //         &self.z
        //     }
        // }

        let p = Point { x: 5, y: 10, z: 15 };
        println!("p.x = {}", p.x());

        let p = Point {
            x: 1.0,
            y: 4.0,
            z: 15.0,
        };
        println!("p.x = {}", p.x());
        println!("p.z = {}", p.distance_from_origin());
    }

    // A method that uses generic types different from its struct's definition
    //
    // The purpose of this example is to demonstrate a situation in which some generic parameters
    // are declared with impl and some are declared with method definition. Here, the generic
    // parameter X1 and Y1 are declared after impl because they go with the struct definition. The
    // generic parameter X2 and Y2 are declared after fn mixup because they're only relevant to the
    // method.
    {
        struct Point<X1, Y1> {
            x: X1,
            y: Y1,
        }

        impl<X1, Y1> Point<X1, Y1> {
            fn mix<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }

        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "hello", y: 'c' };
        let p3 = p1.mix(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }

    // Performance of Code Using Generics
    //
    // Using generic types won't make your program run any slower that it would with concrete
    // types.
    //
    // Rust accomplishes this by performing monomorphization of the code using generic at compile
    // time. Monomorphization is the process of turning generic code into specific code by filling
    // in the conrete types that are used when compiled.
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
