# Bindings and variables

Variables are declared using the `let` keyword as `let <name>:<type-annotation>;` e.g. `let x: i32;`.

**using uninitialised variables will result in an error. Initialisations are done during declarations and bindings immutable by default.*

To allow assignments after initialisation we use `mut` keyword. `let mut x: i32 = 10;` then we can afterwards write `x += 20;`.

The variables respect their scopes and are not accessible outside the block which defines their scope.
```rust
fn main() {
    let x: i32 = 0;
    {
        let y: i32 = 10;
        // this is fine, x and y both are in scope
        prinln!("x is {}, y is {}", x, y);
    }
    // this results in an error since we are accessing y outside its block.
    prinln!("x is {}, y is {}", x, y);
}
```

## Shadowing

Rust allows same variable names to be used again and in the same of inner scopes. The latest declaration will just simply shadow the previous one.

```rust
fn main() {
    let x: i32 = 10;
    let x = x + 10; // shadowing and re-binding
    println!("{}", x); // gives 20
}
```

Types and mutability can be changed when shadowing a variable.
```rust
fn main() {
    let mut x = 30;
    let x = 50;
    x = 20; //error: x is immutable now, following its latest binding.

    let y = 10;
    let y = "I am a string now!";
    println!("{}", x); // gives, I am a string now!
}
```

## Destructuring
We can pull values from a tuple and initialise multiple variables at once.

```rust
fn main() {
    let (w, x) = (1, 3);
    // destructuring supports different states of mutabitlity in declarations.

    let (mut y, z) = (2, "new string"); // tuple can have different types of values.
    y += 10;

    let [a, b] = [1, 2]; // destructuring from an array.
}
```

Destructuring assignments: It allows tuple, array and structure patterns on the left hand side of an assignment.

```rust
fn main() {
    let (x, y);
    // .. discards the rest.
    (x, ..) = (1, 2);
    [.., y] = (3, 4);
    println!("{} {}", x, y); // gives, 1 4
}
```

however if we initialise a variable first and then use destructuring assignments without the `mut` keyword it will result in an error.

## Unit type
The empty type `void` in rust is unit type `()` which means an empty tuple. Any function which does not return anything returns `unit` type represented by an empty tuple `()`.

```rust
fn main() -> () {
    // main returns nothing.
}
```

## Statements and expressions

An expression evaluates like, `x + y` produces a value while a statement does not and it usually ends with a `;`, like a function definition.

In rust a scoped block like below will be considered one expression or statement.

```rust
fn main() {
    // x will be 6
    let x = {
        let y = 2;
        let z = 3;
        y * z // notice the absent ';', this implicitly returns the produced value.
    };

    // y will be unit type ()
    let y = {
        let x = 2;
        let z = 3;
        x * z; // this causes nothing to be returned since the last statement ends with a ';'.
    }

    println!("{} {}", x, y); // error since y is unit type and is not supported directly in println! .
}
```