const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Immutable
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // error: cannot mutate  immutable variable `x`
    // println!("The value of x is: {x}");

    // Mutable
    let mut y = 10;
    println!("The value of y is: {y}");
    y = 15;
    println!("The value of y is: {y}");

    // Constant
    // const are always immutable
    // the type of the value must be annotated
    // can be set only to a constant expression, not the result of a value that could only computed at runtime.
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The number of spaces: {spaces}");
}
