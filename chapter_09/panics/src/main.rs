use std::{
    error::Error as EErr,
    fs::{self, File},
    io::{self, Error, Read},
};

// The Box<dyn Error> type is a trait object,
// For now you can red Box<dyn Error> to mean "any kind of error".
//
// When a main function returns a Result<(), E>, the executable will exit a value of 0 if main
// returns Ok(()) and will exit with a nonzero value if main rreturns an Err value.
fn main() -> Result<(), Box<dyn EErr>> {
    // Unrecoverable Errors with panic!

    // Unwinding the Stack or Aborting in response of Panic
    // By default, when panic occurs the program starts unwinding, which means Rust walks back up
    // the stack and cleans up the data from each functions it encounter. However, walking back and
    // cleaning up is a lot of work. Rust, therefore, allows you to choose the alternative of
    // immediately aborting, which ends the program without cleaning up.
    //
    // Memory that the program was using will then need to be cleaned up by the operating system.
    // If in your project you need to make the resultant binary as small as possible, you can
    // switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate
    // [profile] section in your Cargo.toml file. For example, if you want to abort on panic in
    // release mode, add this:
    //
    // [profile.release]
    // panic = 'abort'

    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    {
        // Recoverable Errors with Result
        // let greeting_file_result = File::open("hello.txt");
        // let greeting_file = match greeting_file_result {
        //     Ok(file) => file,
        //     Err(error) => panic!("Problem opening the file: {error:?}"),
        // };
    }

    {
        // Matching on Different Errors

        // let greeting_file_result = File::open("hello.txt");
        // let greeting_file = match greeting_file_result {
        //     Ok(file) => file,
        //     Err(error) => match error.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //             Err(e) => panic!("Problem creating the file: {e:?}"),
        //         },
        //         other_error => {
        //             panic!("Probleam opening the file: {other_error:?}");
        //         }
        //     },
        // };
    }

    {
        // Alternatives to Using match with Result<T, E>
        // Using closures and the unwrap_or_else method:

        // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        //     if error.kind() == ErrorKind::NotFound {
        //         File::create("hello.txt").unwrap_or_else(|error| {
        //             panic!("Probleam creating the file: {error:?}");
        //         })
        //     } else {
        //         panic!("Problem opening the file: {error:?}");
        //     }
        // });
    }

    {
        // Shortcuts for Panic on Error: unwrap and expect
        // The unwrap method is a shortcut method implemented just like the math expression. If the
        // Result value is the Ok variant, unwrap will return the value inside the Ok. If the
        // Result is the Err variant, unwrap will call the panic! macro for us.

        // let greeting_file = File::open("world.txt").unwrap();

        // Similarly, the expect method lets us also choose the panic! error message. Using expect
        // instead of unwrap and providing good error messages can convery you intent and macke
        // tracking down the source of a panic easier.

        // let greeting_file =
        //     File::open("hello.txt").expect("hello.txt should be included in this project");

        // In production-quality code, most Rustaceans choose expect rather than unwrap and give
        // more context about why the operation is expected to always succeed. That way, if your
        // assumptions are ever proven wrong, you have more inforrmation to use in debugging.
    }

    {
        // Propagating Errors

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
    }

    {
        // A Shortcut for propagating Errors: the ? operator

        fn read_username_from_file() -> Result<String, io::Error> {
            let mut username_file = File::open("hello.txt")?;
            let mut username = String::new();
            username_file.read_to_string(&mut username)?;
            Ok(username)
        }

        // Even shorter version of this code by chaining method calls immediately after the ?.
        fn read_username_from_file_2() -> Result<String, io::Error> {
            let mut username = String::new();

            File::open("hello.text")?.read_to_string(&mut username)?;

            Ok(username)
        }

        // Make the previous code even shorter
        //
        // Reading a file into a string is a fairly common operation, so the standard library
        // provides the convenient fs::read_to_string function that opens the file, creates a new
        // String, reads the content of the file, puts the content into the String, and returns it.
        fn read_username_from_file_3() -> Result<String, io::Error> {
            fs::read_to_string("hello.txt")
        }
    }

    {
        // ? operator on an Option<T> value

        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }
    }

    Ok(())
}
