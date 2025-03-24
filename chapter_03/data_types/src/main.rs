use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not an number!");
    println!("The value of guess: {guess}");

    // Scalar Types
    // A scalar type represnet a single value.
    // Rust has four primary scalar types: integers, floating-poinnt numbers, booleans, and characters.

    {
        // Integer Type
        // integer types default to i32.
        // The primary situation in which you'd use isize or usize is when idexing some sort of collections

        // Integer Overflow
        // If you try to changne the variable to a value outside that range, such as 256, integer
        // overflow will occur, which can result in one of two behaviors.

        // let x: u8 = 256;
    }

    {
        // Floating-Point Types
        // The default type is f64 for floating-point types

        // let x = 2.0; // f64
        // let y: f32 = 3.0; // f32
    }

    {
        // Numeric Operations

        let sum = 5 + 10;

        let difference = 95.5 - 4.3;

        let product = 4 * 30;

        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1

        let remainder = 43 % 5;

        println!("Sum: {sum}");
        println!("Difference: {difference}");
        println!("Product: {product}");
        println!("Quotient: {quotient}");
        println!("Truncated: {truncated}");
        println!("Remainder: {remainder}");
    }

    {
        // The Boolean Type
        let t = true;
        println!("{t}");
        let f: bool = false;
        println!("{f}");
    }

    {
        // The Character Type

        let c = 'z';
        let z: char = 'Z'; // with explicit type annotation
        let heart_eyed_cat = '😻';
        println!("{c}");
        println!("{z}");
        println!("{heart_eyed_cat}");
    }

    // Compound Types
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    {
        // The Tuple Type
        // A tuple is a general way of grouping together a number of values with a variety of types into one
        // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        println!("{}, {}, {}", tup.0, tup.1, tup.2);

        // destructuring
        let (x, y, z) = tup;
        println!("The value of x is: {x}");
        println!("The value of y is: {y}");
        println!("The value of z is: {z}");

        // let tup: () = ();
    }

    {
        // The Array Type
        let _ = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        let mut i = 0;
        loop {
            if i < a.len() {
                println!("{}", a[i]);
                i += 1;
            } else {
                break;
            }
        }

        // let months = [
        //     "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sept", "Oct", "Nov", "Dec",
        // ];

        // You can also initialize an array to contain the same value for each element
        // let a = [3; 5]; // [3, 3, 3, 3, 3]

        println!("{}", a[0]);
        println!("{}", a[1]);

        // Invalid Array Element Access
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }

    {
        // Try
        let names = ["raven", "kristine", "iya", "elia"];

        println!("Please enter an index");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to readline");

        let index: usize = index.trim().parse().expect("invalid index");

        let name = names[index];

        println!("The name with the index of {index} is {name}");
    }
}
