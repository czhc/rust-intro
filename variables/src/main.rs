//ignore unused code
#![allow(dead_code)]
#![allow(unused_doc_comments)]

fn main() {
    let mut x = 5;
    println!("X: {}", x);
    x = 6;
    println!("X: {}", x);

    /**
     * variables vs constants
     * - const are always immutable
     * - const are declared in any scope including global
     * - const can only be set to a constant expression, not result of function call
     * e.g. const MAX_POINTS: i32 = 100_000;
     * but not const MAX_POINTS: i32 = addOne(1)
     * */
     const MAX_POINTS: i32 = 100;
     println!("MAX_POINTS: {}", MAX_POINTS);


     /**
      * shadowing vs mut
      * instead of let mut x = 5; x+1; x*2
      * shadowing / redeclaring the same variable with same name
      * 1. will throw an error if x is _accidentally_ reassigned without `let`
      * */

    let x = 5;
    let x = x+1;
    let x = x*2;
    println!("Value of x: {}", x); // => 12
    /**
     *  2. shadowing allows changing type of variable, while reusing name
     *  if spaces was declared `let mut spaces`, it will throw an error
     *  ```
     *      let mut spaces = "     ";
     *      spaces = spaces.len()
     *  ```
     * => variable does not need to be mutable
     * */

     let spaces = "   ";
     let spaces = spaces.len();
     println!("Length of spaces: {}", spaces);


}

fn add_one(i: i32) -> i32{
    return i+1;
}