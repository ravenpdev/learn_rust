fn main() {
    {
        // Variable Scope
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
        println!("{s}");
    } // this scope is now over, and s is no longer valid

    {
        // The String Type
        let mut s = String::from("hello"); // s is valid from this point forward
        s.push_str(", world!");
        println!("{s}");
    } // this scope is now over, and s is no longer valid.

    {
        // Variables and Data Interacting with Move
        // A string is made up of three parts, a pointer to the memoery that holds the content of
        // the string, a length, and a capacity. This group of data is stored on the stack.
        // the content is stored in the heap.
        //
        // the length is how much memory, in bytes, the content of the String are currently using.
        // the capacity is the total amount of memory, in bytes that the String has received from
        // the allocator.
        let s1 = String::from("hello");
        let s2 = s1;
        println!("{s2}");
    }

    {
        // Variables and Data Interacting with clone
        // If we do want a deeply copy the heap data of the String, not just the stack data, we can
        // use a common method clone.
        // when you see a call to clone, you know that some arbitrary code is being executed and
        // that code maybe expensive.
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
    }

    {
        // Onwerships and functions
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s' value moves into the function...
        //                  .. and so is no longer valid here
        // println!("{s}");

        let x = 5; // x comes into scope
        makes_copy(x); // because i32 implements the Copy trait, x does not move into the function
        println!("{x}"); // so it is okay to use x afterwards
    }

    {
        // Return values and scopes
        let s1 = gives_ownership(); // gives_ownership move its return value to s1
        println!("{s1}");

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2);

        println!("{s3}");
    }

    {
        // While this works, taking ownership and then returning ownership with every function is a
        // bit tedious.
        // Rust does let us return multiple values using a tuple,
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of `{s2}` is {len}.");
    }
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
