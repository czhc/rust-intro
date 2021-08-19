# Data Types


## Scalar Types

* Integers (signed (+/-) or unsigned (+ only)): `i` or `u`:
    * `i8` stores: `-2^7` to `2^7-1` i.e. `-128`-`127`
    * `u8` stores `0` to `2^8-1` i.e. `0`-`255`
    * types depend on cinoyter type: 64bits on 64-bit architecture, vice versa for 32-bit
    * _integer overflow_ e.g. when declaring `let foo: u8 = 256`. throws `literal out of range`
* Floating point: `f32` or `f64` only. default `f64`
* Character type: `char` literals use single quotes. string literals use double-quotes.


## Compound Types

* tuples: `let tup: (i32, f64, u8) = (500, 6.4, 1)` and to unpack/destructure `let (x,y,z) = tup`
    * acccess element by index using `.`: `tup.0` => `500`
* arrays:  data allocated on the stack rather than the heap
    * arrays are fixed length, vectors can grow/shrink
    * array type declaration `[type, length]`: `let a: [i32;5] = [1,2,3,4,5]`
    * accessing index outside of array throws _out of bounds_ error.
    * rust checks index and panics as a safety principle. in other low-level languages, without this check, it is possible to access invalid memory via wrong index.

