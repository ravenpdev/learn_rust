use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    {
        // Closure type inference and annotations
        // let expensive_closure = |num: u32| -> u32 {
        //     println!("Calculating slowly...");
        //     thread::sleep(Duration::from_secs(2));
        //     num
        // };

        // fn add_one_v1(x: u32) -> u32 {
        //     x + 1
        // }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x| { x + 1 };
        // let add_one_v3 = |x: u32| x + 1;

        {
            // The first time we call example_closure with the String value, the compiler infers
            // the type of x and return type of the closure to be String. Those types are locked
            // into the closure in example_closure, and we get a type error when we try to use a
            // different type with the same closure.

            // let example_closure = |x| x;
            // let s = example_closure(String::from("hello"));

            // let n = example_closure(5); // error: mismatched type
        }

        // Capturing references or Moving ownerships

        // {
        //     // Closure that captures immutable reference to the vector named list because it only
        //     // needs an immutable reference to print the value:
        //     let list = vec![1, 2, 3];
        //     println!("Before defining closure: {list:?}");

        //     let only_borrows = || println!("From closure: {list:?}");

        //     println!("Before calling closure: {list:?}");
        //     only_borrows();
        //     println!("Afterr calling closure: {list:?}");
        // }

        // {
        //     // Capture mutable reference
        //     let mut list = vec![1, 2, 3];
        //     println!("Before defining closure: {list:?}");

        //     let mut borrows_mutably = || list.push(7);

        //     // println!("Before calling closure: {list:?}");
        //     borrows_mutably();
        //     println!("After calling closure: {list:?}");
        // }

        // {
        // If you want to force the closure to take ownership of the values it uses in the
        // environment even though the body of the closure doesn't strictly need ownership, you
        // can use the move keyword before the paramter list.
        //
        // This technique is useful when passing a closure to a new thread to move the data so
        // that it's owned by the new thread.

        // let list = vec![1, 2, 3];
        // println!("Before defining closure: {list:?}");

        // thread::spawn(move || println!("From thread: {list:?}"))
        //     .join()
        //     .unwrap();
        // }

        // {
        //     let list = vec![1, 2, 3];
        //     println!("Before defining closure: {list:?}");

        //     let move_value = move || list;

        //     println!("After calling closure: {:?}", move_value());
        // }
    }

    {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });

        println!("{:#?}", list);
        println!("{}", num_sort_operations);
    }
}
