# Ownership

- Every value will have an owner.
- There can be only one owner of a value at once.
- If the owner goes out of scope it is freed.

## Copy and move

Any literal or fixed length value is stored on stack and is copied because it is cheap. Like `i32`, `u32`, `i8`, `bool`, `char` etc.

However any value stored on heap, whose size we do not know is moved whenever passed among variables. Since copy will be expensive here the value is moved to the new owner and the previous is freed.

```rust
fn main() {
    let s1 = String::from("test string");
    println!("{}", s1); // s1 is the owner and has access.
}
```

```rust
fn main() {
    let s1 = String::from("test string");
    let s2 = s1; // value is moved to s2 after assigment.
    println!("{}", s1); //error: s1 is not the owner now. 
}
```

To create a copy we have to invoke `clone` method explicitly.

```rust
fn main() {
    let s1 = String::from("test string");
    let s2 = s1.clone(); // value is copied to s2 after assigment.
    println!("{}", s1, s2);
}
```

## Ownership and functions

Heap references passed to the functions are moved and the original variable holding the value will no longer have access to it after the function call.

```rust
fn main() {
    let s1 = String::from("test string");
    printStr(s1);
    println!("{}", s1); // error, since value is moved to parameter str.
}

fn printStr(str: String) {
    println!("{}", str);
}
```

## Mutability

Mutability can be changed when transferring ownership

```rust
fn main() {
    let s1 = "test string".to_string();
    let mut s2 = s1;
    
    // if we call push_str on s1 it will give an error since it is immutable.
    s2.push_str(" with more data");

    println!("{}", s2);
}
```

## Partial move

When destructuring values from another data structure, we can selectively take ownership of members, only the members which have not been moved can be accessed through the original variable.

```rust
struct Test {
    x1: String,
    x2: String
}

fn main() {
    let test = Test {
        x1: "test1".to_string(),
        x2: "test2".to_string()
    };

    // destructuring values to another struct value of the same type. Note the ref keyword.
    let Test { x1, ref x2} = test;

    println!("{} {} {}", x1, x2, test.x2); // test.x2 is still accessible.
}
```

We can partially move values in tuples too

```rust
fn main() {
    let x = ("s1".to_string(), "s2".to_string());
    let y = x.0;
    println!("{} {}", y, x.1);
}
```

## Borrowing

Borrowing is a way to have temporary access to a value without taking ownership. It is achievable using references which can be mutable or immutable.

While borrowing a variable certain rules are to be followed.

- You can take one mutable reference or several immutable references at once.
- A reference must always be valid.

```rust
fn main() {
    let s1 = "test string".to_string();
    let s1Ref = & s1 // immutable reference.
    println!("{}", s1Ref);
}
```

```rust
fn main() {
    let s1 = "test string".to_string();
    let s1Ref = &mut s1 // immutable reference.
    s1.push_str(" with more values");
    println!("{}", s1Ref);
}
```

Creating mutable and immutable references together is illegal.

It is possible to create mutable and immutable references of a varibale in one scope, however, care must be taken to not use the mutable reference after an immutable reference is declared and vice-versa.

```rust
fn main() {
    let mut s1 = "test string".to_string();
    let s1Mut = &mut s1;
    println!("{}", s1Mut);

    let s1Ref = & s1;
    println!("{}", s1Ref);

    s1Mut.push_str("test s"); // error
}
```

*Dangling references are not allowed, rule 2*

```rust
fn main() {
    let s = dangle;
    println!("{}", s);
}

fn dangle() -> &String {
    let s = "test string".to_string();
    &s // s will be removed after this scope.
}
```

**we can also use `ref` keyword instead of `&`, they are similar**