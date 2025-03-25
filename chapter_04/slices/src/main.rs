fn main() {
    {
        let s = String::from("Hello world");

        let word = first_word(&s[..]); // word will get the value 5

        // s.clear(); // this empties the String, making it equal to ""

        println!("{word}")
        // `word` still has the value of `5` here, but `s` no longer has any content
        // that we could meaningfully use with the value `5`, so `word` is now totally invalid!
    }

    {
        // String Slices
        // A String slice is a reference to part of a String,
        // Internally, the slice data structure stores the starting position and the length of the
        // slice, which corresponds to ending_index minus starting_index.

        let s = String::from("hello world");

        let hello = &s[0..=4];
        let world = &s[6..11];

        println!("{hello} {world}");
    }

    {
        let my_string = String::from("hello world");

        let word = first_word(&my_string[0..6]);
        println!("{word}");
        let word = first_word(&my_string[..]);
        println!("{word}");

        // `first_word` also works on references to `String`s, which are equivalent to whole slices
        // of `String`s
        let word = first_word(&my_string);
        println!("{word}");

        let my_string_literal = "hello world";
        let word = first_word(&my_string_literal[0..6]);
        println!("{word}");
        let word = first_word(&my_string_literal[..]);
        println!("{word}");

        // Because string literal are string slices already.
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
        println!("{word}");
    }

    {
        // Other slices
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &str) -> &str {
    // &str signifies "string slice"
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
