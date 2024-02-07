# Functions
A function definition follows:
```rust
fn <name> (param1: type, param2: type, ...) -> return_type {
    // definition
}
```

**If the function returns something omit ';' in the last statement and it will return whatever is produced by the expression, make sure the type match though.*

Example:
```rust
fn main() {
    let x = 10;
    let y = 20;

    let sum = sumVars(x, y);
    
    println!("{}", sum); // prints 30
}

fn sumVars(x: i32, y: i32) -> i32 {
    x + y
}
```

## Diverging functions

A function which never returns to its caller.

```rust
fn main() {
    neverReturn();
    println!("failing"); // a compiler warning will be shown saying unreachable statement.
}

fn neverReturn() -> ! {
    // these three macros exit the program, causing the function to never return.
    panic!("program in panic mode.");
    // unimplemented!()
    // todo!()
}
```