# Variables

Variables vs Constants:

- const are always immutable
- const are declared in any scope including global
- const can only be set to a constant expression, not result of function call

e.g. `const MAX_POINTS: i32 = 100_000`
but not `const MAX_POINTS: i32 = addOne(1)`


Shadowing vs `mut`:

1. will throw an error if x is _accidentally_ reassigned without `let`
2. shadowing allows changing type of variable, while reusing name. If variable was declared `let mut spaces`, it will throw an error