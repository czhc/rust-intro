# Functions

* Functions with return values: `fn five() -> i32 { ...}`
* most functions return the last expression implicitly

```rust
fn plus_one(x: i32) -> i32 {
    //x+1; is invalid due to ';'
    x+1
}

```

`x+1;` is invalid. Error: `implicitly returns `()` as its body has no tail or `return` expression`

`x+1;` is a `statement`, and is not returning a `i32`


# Conditionals

Conditional blocks

```rust
if (condition1) {

} else if (condition2) {

} else {

}

```
Conditional assignments: requires same value
```rust
    //valid
    let number = if condition { 5 } else  { 6 };

    //invalid
    let number = if condition { 5 } else { 'foo' };

```

# Loops

for-loop statement:

```rust
for e in a.iter() {
    // code
}

```

`iter()` is function of _iterable Traits_. other functions of iteration are `iter_mut()` and `into_tier()`

`