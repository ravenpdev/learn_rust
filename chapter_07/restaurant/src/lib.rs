use std::fmt;
use std::io;

mod front_of_house;

// Starting relative paths with super
mod back_of_house {
    // making an enum public, makes all its variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {
        super::front_of_house::hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

// pub use crate::front_of_house::hosting;
// use crate::front_of_house::hosting;
//
pub use crate::front_of_house::hosting;

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let _order1 = super::back_of_house::Appetizer::Soup;
        let _order2 = super::back_of_house::Appetizer::Salad;

        // Absolute path
        // crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        // front_of_house::hosting::add_to_waitlist();

        // Brinnging Paths into Scope with the use Keyword
        hosting::add_to_waitlist();
    }
}

// fn function1() -> fmt::Result {}

// fn function2() -> io::Result<()> {}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
