fn main() {
    {
        // Reference
        // A reference is like a pointer in that it's an address we can follow to access the data
        // stored at that address; that data is owned by some other variable. Unline a pointer, a
        // reference is guaranteed to point to a valid value of a particular type for the life of
        // that reference.
        let s1 = String::from("Hello");

        let len = calculate_length(&s1);

        println!("The length of `{s1}` is {len}.");
    }

    {
        // Dangling references
        // let reference_to_nothing = dangle();
    }

    {
        // Mutable references
        let mut s = String::from("hello");
        change(&mut s);
        println!("{s}");

        // Mutable references have one big restriction: if you have a mutable referenes to a value,
        // you can have no other references to that value.
        // let mut s = String::from("hello");
        // let r1 = &mut s;
        // let r2 = &mut s;
        // println!("{r1} {r2}");

        // The Rules of References
        //
        // - At any given time, you can have either one mutable references or any number of
        // immutable references.
        // - References must always be valid.
    }
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }
