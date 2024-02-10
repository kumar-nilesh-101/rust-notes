# Arrays

Arrays are fixed size contiguous structures which store a single type of data. Type of an array is `[T; length]`.

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // single array type and fixed length.
    let b = [1, 2, 3, 4, 5]; // type of an array will be inferred by the compiler as well.
}
```

## Index based access

Square bracket index based access is available in rust as well `<array_name>[index]`.

Or, `get(index: usize)` method which also provides index based access, though in a safer way in case index is out of bounds.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let i = 0;
    let j = get_index();

    println!("{}", a[i]);
    println!("{}", a[j]); // this will panic at runtime.
}

fn get_index() -> usize {
    10
}
```
Using `get(index: usize) -> Option<T>`

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let val1 = a.get(1).unwrap();
    /*
        this way provides a safer way to access arrays at runtime, since
        Option<T> return type can default to a defined value if the match fails.
    */
    let val2 = a.get(10).unwrap();
}
```

*Rules of ownership also apply on arrays of reference types. Arrays thus do not allow simple access in assignment.*

```rust
fn main() {
    let a = [String::from("test"),String::from("test"),String::from("test"),];

    let x = a[0]; // invalid
    // works
    let y = a[0].clone();
    let ref z = a[0];
}
```

## Slices

Similar to a string slice, provides a read only reference.

```rust
fn main() {
    let a = [1, 2, 3, 4];
    let slice = &a[0 .. 2];
}
```