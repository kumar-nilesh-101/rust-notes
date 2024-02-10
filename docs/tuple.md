# Tuple

A tuple is a fixed length collection of different types of values represented as `(T1 .. Tn)`.

Tuples allow index based access using the dot `.` operator.

Tuples support destructuring access and assignments, we can use them as function arguments and return values as well.

```rust
fn main() {
    let t: (u32, f64, i32) = (1, 1.54, -23);
    
    // destructuring access
    let (x, y, ..) = t;

    // destructuring assignment
    let (a, b);
    (a, b, ..) = t; 
}
```

*When tuples have values stored on heap, the ownership rules of moving and borrowing applies all the same.*

## Index based access

```rust
fn main() {
    let a = (1, 2, 3);
    let x = a.0;
    let y = a.1;
}
```

## Using as function arguments and return value

```rust
fn main() {
    let (sum, product) = sum_mult((2, 3));
    println!("{} {}", sum, product); // give 5 6.
}

fn sum_mult(v: (i32, i32)) -> (i32, i32) {
    (v.0 + v.1, v.0 * v.1)
}
```