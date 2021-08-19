// disables unused_doc_comments warning in linter
#![allow(unused_doc_comments)]

/**
 * rust prelude only includes some modules: https://doc.rust-lang.org/std/prelude/index.html
 * use `use` to add out of scope packages
 * */

use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your guess: ");

    /**
     * `let` declares a variable. all variables are immutable
     * unless declared with `mut`
     * `::` indicates function `new` from type `String`
     * `String::new` produces empty string
     * */

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    /**
     `&` indicates arg is a reference.rust-lang
     access same piece of data without need to copy data into mem multiple times
    **/

    /**
     * read_line returns an `io::Result`. Rust has a general `Result` type, or submodules e.g. `io::Result`
     * `Result types are enums: either `Ok` or `Err` to encode error-handling
     * `io::Result` has an `expect()` method.
     * if `io::Result` is an `Err`, `expect` throws and display given arg
     * if `io::Result` is an `Ok`, `expect` takes the return value of `Ok` and returns only the value.
     * Rust will raise a warning if `Result` values are not handled.
     * */

    println!("You guessed: {}", guess);

    //print values are ordered and interpolated with {}
}
