# Struct

A `struct` is a collection of values different types just like a tuple, though `struct`s associate values with names instead of index.

```rust
struct Person {
    name: String,
    age: u32
}

fn main() {
    let person = Person {
        name: String::from("John Doe"),
        age: 30
    };
}
```

We need to define concrete values for each of the fields in `struct`s.

There are several ways to define values of fields of a `struct` without being too vebose.

```rust
struct Person {
    name: String,
    age: u32
}

fn main() {
    let name = String::from("Jane Doe");
    let age = 31;

    let person1 = Person {
        name: String::from("John Doe"),
        age: 30
    };

    let person2 = Person {
        name, // shorthand if variable name and property name is same.
        age
    };

    let person3 = Person {
        name: String::from("Jack Sparrow"),
        ..person1 // it sets the rest of the values from another struct.
    };
}
```

## Tuple structs

A tuple struct is defined as `struct <name>(...fields)`. They provide index based access to the fields.

```rust
struct Color(u32, u32, u32);

fn main() {
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    println!("rgb value of color black is R: {}, G: {}, B: {}", black.0, black.1, black.2);
}
```

*A `struct` should be instantiated with `mut` keyword if values of the fields need to be changed later on.*