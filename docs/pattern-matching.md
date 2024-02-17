# Pattern matching

In Rust there is a `match` construct available which is similar to `switch` statement. However it is much more powerful because it supports proper pattern matching. We can use it with variable values(the traditional thing), wildcards, literals etc.

*It requires every possible case to be handled. It is enforced the by compiler.*

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main() {
    let cents = get_cents(Coin::Nickel);
    println!("{}", cents);
}

fn get_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
```

## `if let` statement

It is a concise way to write a `match` statement if the value of variable is known.

```rust
fn main() {
    // Option type handles empty values well but they are wrapped up.
    let x: Option<i32> = Option::Some(5);
    
    // we need to check for specific value we need then only perform the operation.
    let double_if_not_null = match x {
        Some(5) => 2 * 5,
        _ => 0
    };

    println!("{}", double_if_not_null);
}
```

This can be achieved using a simple `if let` statement.

```rust
fn main() {
    let x = Option::Some(5);
    
    let double_if_not_null = if let Some(valid) = x {
        valid * 2
    } else {
        0
    };

    println!("{}", double_if_not_null);
}
```

Thus, we avoided all the boilerplate code we had to deal with previously to unwrap the `Option` type and simply destructured the value into `v` and checked.

We can check for multiple distinct values in one line or a range

```rust
fn main() {
    let x = Option::Some(5);
    let y = 982;
       
    let in_string = match x {
        Some(5) | Some(6) | Some(7) => "5,6,7",
        _ => "something else"
    };

    let in_string_range = match y {
        1..=1000 => "1k",
        _ => "more than 1k"
    };

    println!("{}", in_string);
    println!("{}", in_string_range);
}
```

We can destructure values on the fly to handle complex structures in `match` statment.

```rust
enum Message {
    Point (i32, i32),
    Move { x: i32, y: i32},
    ChangeColor (u8, u8, u8)
}

fn main() {
    let message = Message::ChangeColor(0, 0, 0);
    match message {
        Message::Point(x, y) => {
            println!("Message is point at {}, {}", x, y);
        },
        Message::Move {x: a, y: b} => {
            println!("Message is move to {}, {}", a, b);
        },
        Message::ChangeColor(r, g, b) => {
            print!("Message is change color to {}, {}, {}", r, g, b);
        }
    }
}
```

## `@` operator

It lets us create a variable which holds a value at the same time we're matching it with given values or a range.

```rust
fn main() {
    let point = Point {
        x: 2,
        y: 20
    };

    match point {
        Point { x, y: 0 } => println!("on x axis"),
        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("still on x axis"),
        _ => println!("can be anywhere")
    }
}
```
```rust
enum Coin {
    NoDollar { value: i32 },
    Dollar { value: i32 }
}

fn main() {
    let quarter = Coin::NoDollar { value: 25 };

    match quarter {
        Coin::NoDollar {
            value: val @ 0..=99
        } => println!("we have less than a dollar"),

        _ => println!("we have more money")
    }
}
```

### Additional complexity

Within `match` branches an `if` can be used to check if the branch can be executed or not.

```rust
enum Coin {
    NoDollar { value: i32 },
    Dollar { value: i32 }
}

fn main() {
    let quarter = Coin::NoDollar { value: 25 };

    match quarter {
        Coin::NoDollar {
            value: val @ 0..=99
        } if val == 25 => println!("we have less than a dollar, specifically a quarter"),

        _ => println!("we have more money")
    }
}
```

### Ignoring values using `..`

```rust
fn main() {
    let pow2 = (1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match pow2 {
        (first, .., last) => {
            assert_eq!(first, 1);
            assert_eq!(last, 2048)
        }
    }

    println!("success");
}
```

