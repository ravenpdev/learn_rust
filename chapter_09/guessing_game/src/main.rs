use std::{cmp::Ordering, io};

use rand::Rng;

mod user_define {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        println!("Enter guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(v) => v,
            _ => continue,
        };

        let guess = user_define::Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("too low!"),
            Ordering::Greater => println!("too hight!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
