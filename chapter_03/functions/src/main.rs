fn main() {
    println!("Hello, world!");

    another_function();

    print_value(10);

    print_labeled_measurement(5, 'h');

    // Statement and Expressions
    // Statements are instructions that perform some action and do not return value.
    // Expressions evaluate to a result value.

    let _ = 6; // statement

    let y = {
        let x = 3;
        x + 1 // no semicolon here will return the value
    }; // expression
    println!("The value of y is: {y}");

    let five = five();
    println!("{five}");
}

fn another_function() {
    println!("Another function.");
}

fn print_value(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
