fn main() {
    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("{s3}");
    // The reason s1 is no longer valid after the addition, and the reason we use a reference to
    // s2, has to do with the signature of the method that's called when we use the + operator. The
    // + operator uses the add method, whose signature looks like this:
    //
    // fn add(self, s: &str) -> String {}

    // For combining strings in more complicated ways, we can instead use format! macro.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let hello = "Зд";
    let s = &hello[0..4];
    println!("{s}");

    for c in hello.chars() {
        println!("{c}");
    }

    for b in hello.bytes() {
        println!("{b}");
    }
}
