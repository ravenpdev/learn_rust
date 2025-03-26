use std::any::Any;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // creating an instance of a struct
    //
    // Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain
    // fields as mutable.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someusername123@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("someuser@examp.com");

    println!("{}", user1.email);

    let user2 = build_user("ravenpdev@gmail.com".to_string(), "raven".to_string());
    println!("{}", user2.email);

    // Creating Instances from Other Instances with Struct Update Syntax
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user1
    };
    println!("user3 email: {}", user3.email);
    println!("user1 is active: {}", user1.active);
    // println!("user1 username: {}", user1.username); // we can no longer use user1 as a whole after
    // creating user3 because the String in the username field of user1 was moved into user2.

    {
        // Tuple Structs
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("black {}, {}, {}", black.0, black.1, black.2);
        println!("origin {}, {}, {}", origin.0, origin.1, origin.2);

        // Destructuring tuple struct
        let Point(x, y, z) = origin;
        println!("x: {}, y: {}, z: {}", x, y, z);
    }

    {
        // Unit-Like Structs Without Any Fields

        // struct AlwaysEqual;
        // let subject = AlwaysEqual;
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
