# Generics

Generics provide a way to define placeholders for concrete types. They are more flexible and reduce duplicity.

Generics are zero cost abstractions which are replaced by concrete types at compile time by the rust compiler.

```rust
struct A; // concrete type
struct S(A); // concrete type
struct SGen<T>(T); // generic type

fn main() {
    let a = A;
    let s = S(a);
    let sgen = SGen(A{});
}
```

We can define implementation blocks for a generic type and provide concrete types to work with.

```rust
struct Point<T> {
    x: T,
    y: T
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // explicitly providing the type
    let int_points: Point<i32> = Point {
        x: 10,
        y: 10
    };

    // inferred type
    let float_point: Point<f32> = Point {
        x: 1.10123,
        y: 2.23522
    };

    println!("distance from origin of the point is {}", float_point.distance_from_origin());
}
```

Or, we can have an `impl` block which can work with generic types.

```rust
struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn mix_x_y<V, W>(self, p: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: p.y
        }
    }
}

fn main() {
    // explicitly providing the type
    let int_points: Point<i32, i32> = Point {
        x: 10,
        y: 10
    };

    // inferred type
    let float_point: Point<f32, f32> = Point {
        x: 1.10123,
        y: 2.23522
    };

    let mixed_point = float_point.mix_x_y(int_points);

    println!("mixed point x is {}, y is {}", mixed_point.x, mixed_point.y);
}
```