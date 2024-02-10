# Strings

A string is defined as a sequence of characters. In rust, we can have mutable strings defined by `String` and immutable string views using `str`.

## std::String

A String is a dynamic structure holding a reference to a heap allocated block which stores the string data.

```rust
fn main() {
    let mut s1: String = String::from("test string");
    // we have methods to change the data like push, push_str

    s1.push('&'); // single character
    s1.push_str(" test"); // another string
}
```

## std::str

An str represents a string literal which is harcoded in the binary. This literal is immutable and is a view only container.

```rust
fn main() {
    let s: &str = "hello world!";
    let s1 = s;

    println!("{} {}", s, s1); // no ownership issues since a reference is passed during the assignment.
}
```

## Concatenation

We can concatenate string using `+` operator, however every argument following the first `String` type argument must be of type `&str`.

```rust
fn main() {
    let s = String::from("hello world");
    let s1 = String::from(" I am a dev");

    let s_s1 = s + &s1; // a reference to a String also works fine;

    println!("{} {}", s, s1);
}
```

However, after concatenation `s` will be moved to `s_s1` and will no longer be accessible.

## String slices

A string slice is also represented using `&str`, they are view only.

```rust
fn main() {
    let s1 = String::from("hello world!");
    // note & before accessing via indices
    let slice = &s1[0..5];
    println!("{}", slice); // prints hello
}
```