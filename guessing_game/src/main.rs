// disables unused_doc_comments warning in linter
#![allow(unused_doc_comments)]

/**
 * rust prelude only includes some modules: https://doc.rust-lang.org/std/prelude/index.html
 * use `use` to add out of scope packages
 * */

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number (between 1-10)!");
    let secret = rand::thread_rng().gen_range(1..10);

    loop {

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

        /**
         * Rust allows us to shadow the previous value of `guess` with a new one.
         * Use this for value conversions - reuse variable name instead of creating two unique variables
         * `u32` - unsigned 32-bit int, `i32` - 32bit number. number types from 1-100.
         * default number type is `i32`
         * */

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);
        //print values are ordered and interpolated with {}

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
        }
    }

    println!("The secret is: {}", &secret);
}
