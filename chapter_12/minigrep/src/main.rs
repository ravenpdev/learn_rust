use std::{env, process};

use minigrep::{Config, run};

fn main() {
    // Iterators
    //
    // Iterator produce a series of values, and we can call the collect method on an iterator to
    // turn it into a collection, such as vector, that contains all the elements the iterator
    // produces.

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    println!("Search for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
