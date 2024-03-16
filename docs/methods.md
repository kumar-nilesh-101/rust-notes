# Methods

A method is a function associated to a particular type or struct. They are defined inside an `impl` block as `impl Struct_name {}`. Any method definitions inside this block will accessible using the `struct`'s name.

A method accepts `&self` parameter which is a reference to the instance with which the method is called.

```rust
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect = Rectangle {
        length: 10,
        width: 10
    };

    println!("area of the rectangle is {} units", rect.area());
}
```

# Associated functions

An associated function is all the methods inside an `impl` block. Some methods can be defined without the `&self` parameter. Such methods are not called on an instance but on the type itself.

```rust
struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn new(length: u32, width: u32) -> Rectangle {
        Rectangle {
            length,
            width
        }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }
}

fn main() {
    let rect = Rectangle::new(10, 10);
    println!("area of the rectangle is {} units", rect.area());
}
```

*`fn method(&self)` is just syntactic sugar for `fn method(self: &self)`.*

A type can have several `impl` blocks, it is useful for reorganizing stuff.

## Associated methods with an `enum`

```rust
enum Color {
    Red,
    Green,
    Blue
}
impl Color {
    fn printColor(&self) -> &str {
        match self {
            Color::Red => "red",
            Self::Green => "green", // instead using Type::property we can use Self to refer to the type.
            Self::Blue => "blue"
        }
    }
}

fn main() {
    let color = Color::Red;
    println!("Chosen color is {}", color.printColor());
}
```