fn main() {
    println!("Hello, world!");

    {
        // If expression
        let number = 3;

        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }

        if number != 0 {
            println!("number was something other than zero.");
        }
    }

    {
        // Handling multiple condition with else if
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    {
        // Using if in a let statement
        let condition = true;

        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }

    {
        // Repitition with loops
        let mut i = 0;
        loop {
            if i == 10 {
                break;
            }

            println!("again!");
            i += 1
        }
    }

    {
        // Returning values from loops
        // You can also return from inside a loop, While break only exits the current loop, return
        // always exits the current function

        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {result}");
    }

    {
        // Loops labels to disambiguate between multiple loops
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");

            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");

                if remaining == 9 {
                    break;
                }

                if count == 2 {
                    break 'counting_up;
                }

                remaining -= 1;
            }

            count += 1;
        }

        println!("End count = {count}");
    }

    {
        // Conditional loops with while
        let mut number = 3;

        while number != 0 {
            println!("{number}!");
            number -= 1;
        }
        println!("LIFTOFF!!!");
    }

    {
        // Looping through a collection with for
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("The value is: {}", a[index]);
            index += 1;
        }
    }

    {
        // Using for loop
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("The value is: {element}");
        }

        for number in (1..4).rev() {
            println!("{number}");
        }
        println!("LIFTOFF!");
    }

    let celcius = fahrenheit_to_celcius(100.0);
    println!("Celcius is: {celcius}");

    let fahrenheit = celcius_to_fahrenheit(celcius);
    println!("Fahrenheit is: {fahrenheit}");

    fibonacci(5);

    twelve_day_of_christmas();
}

fn fahrenheit_to_celcius(value: f64) -> f64 {
    (value - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(value: f64) -> f64 {
    (value * (9.0 / 5.0)) + 32.0
}

fn fibonacci(value: i32) {
    print!("0 1 1 ");

    let mut prev = 1;

    for outer in 2..(value - 1) {
        print!("{} ", prev + outer);
        prev = outer;
    }
    println!()
}

fn twelve_day_of_christmas() -> () {
    const TWELVE_DAYS: [&str; 12] = [
        "a partridge in a pear tree!",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swanns a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    const DAY_IN_WORDS: [&str; 12] = [
        "first", "second", "thrid", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in 1..=12 {
        println!(
            "On the {} day of christmas my true love gave to me",
            DAY_IN_WORDS[day - 1]
        );

        for repeat in (0..day).rev() {
            if day != 1 && repeat == 0 {
                print!("and ");
            }
            println!("{}", TWELVE_DAYS[repeat]);
        }
    }
}
