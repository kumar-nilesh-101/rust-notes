# enum

An `enum` is an enumerated collection of different variants of a single custom type. Enumeration is automatic, but we can define the type of value a variant can hold as well. It can be tuple like or struct like.

A variant in `enum` is one of the possible values out of the definite set of values it defines.

```rust
// Number is a custom type, to print it we use Debug macro.
#[derive(Debug)]
enum Number {
    ZERO, // starts enumeration with zero
    ONE = 10,
    TWO // it will have 11 since we set ONE to 10.
}

fn main() {
    // an enum value can converted to number using as keyword
    println!("{:?}", Number::ZERO as u8); // gives 0.
    println!("{:?}", Number::ONE); // gives ONE.
    println!("{:?}", Number::TWO as u8); // gives 11.
}
```
In an `enum` we define only one of the variants unlike `struct` where we need to provide concrete values for every field.

Every variant can hold its own data, in a tuple or `struct` or any other type; primitive or non-primitive. The type of the variant can be independent of other variants. Every variant will be considered of the type enum.

Tuple like `enum` variant. 

```rust
#[derive(Debug)]
enum IP {
    v4(String),
    v6(String)
}

fn main() {
    let v4Addr = IP::v4(String::from("127.0.0.1"));
    let v6Addr = IP::v6(String::from("::1"));
    
    println!("{:?}", v4Addr);
    println!("{:?}", v6Addr);
}
```
Variants with different types.

```rust
#[derive(Debug)]
enum IP {
    v4(String, String),
    v6(String, String)
}

#[derive(Debug)]
enum ConnectionDetails {
    ip_addr (IP),
    location { lat: i32, long: i32 }
}

fn main() {
    let v4Addr = IP::v4(String::from("127.0.0.1"), String::from("v4"));
    let v6Addr = IP::v6(String::from("::1"), String::from("v6"));

    let connectionDetails = ConnectionDetails::ip_addr(v4Addr);
    let connectionLocationDetails = ConnectionDetails::location { lat: -1, long: 1 };

    println!("{:?}", connectionDetails);
    println!("{:?}", connectionLocationDetails);
}
```

## Option

Since rust does not have an equivalent of null value. An `Option` type represents something which might be null. For example an out of bounds index access of an array. This provides a safer way to handle absence of value.

```rust
enum Option<T> {
    None,
    Some (T)
}
```

`Option::None` is absence of value while `Option::Some(T)` represents a valid value of type `T`.

```rust
fn main() {
    let one = Option::Some(1);
    let two = plus_one(one);
    let none = plus_one(Option::None);

    if let Some(n) = two {
        println!("{}", n);
    } else {
        panic!("no value found");
    }
}

fn plus_one(v: Option<i32>) -> {
    match v {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1)
    }
}
```

*`if let` is used to unwrap a value from the enum variant, prvides a safer conditional access only if the value exists.*
