#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Alaska);
    let cent = value_in_cents(coin1);

    println!("cent value: {cent}");

    {
        // Matching with Option<T>

        let five = Some(5);
        let _six = plus_one(five);
        let _none = plus_one(None);
    }

    {
        // Catch-all pattern and the _ placeholder
        let dice_roll = 9;

        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }

        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (), // Here, we're telling rust explicitly that we aren't going to use any other
                     // value that doesn't match a pattern in an earlier arm, and we don't want to run any
                     // code
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Lucky penny!");
            5
        }
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_space: u8) {}
fn reroll() {}
