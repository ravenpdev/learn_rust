use std::fmt::Display;

fn main() {
    // Validating References with Lifetimes
    //
    // Lifetimes are another kind of generic that we've already been using. Rather than ensuring
    // that a type has the behavior we want, lifetimes ensures the references are valid as long as
    // we need them to be.

    // Preventing dangling references with lifetimes
    {
        let r; // ----- 'a

        let x = 5; // ---- 'b
        r = &x;

        println!("r: {r}");
        // ----- 'b
    } // ----- 'a

    // Generic Lifetimes in Functions
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
        println!("string1: {string1}");
        println!("string2: {string2}");
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }

    s2
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");

    if x.len() > y.len() {
        return x;
    }

    y
}
