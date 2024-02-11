# Control flow

## if-else

```rust
fn main() {
    let x = 10;

    if(x == 10) {
        println!("x is {}", x);
    } else {
        println!("invalid value");
    }
}
```
We can also use `if-else` statements for initialisation.

```rust
fn main() {
    let x = 10;
    let y = if (x < 10) {
        x * x
    } else {
        x
    }; // should be followed by a ;, if used in initialisation.
}
```

## Loop statements

- `for`
- `while`
- `loop`

### `for` loop

```rust
fn main() {
    // creating a range and running a for loop.
    // omit '=' to exclude the end. 
    for i in 1 ..= 100 {
        if(i > 100) {
            panic!("value range exceeding");
        } else {
            print!("{}", i);
        }
    }
}
```

Iterating on an array or a range along with indices. Otherwise a `for in` loop will suffice.

```rust
fn main() {
    let a = [9, 8, 7, 6, 5];

    for (v, i) in a.iter().enumerate() {
        println!("{} {}", v, i);
    }

    let r = 1 ..= 100;

    for (v, i) in r.enumerate() {
        println!("{} {}", v, i);
    }
}
```

### `while` loop

```rust
fn main() {
    let arr = [3, 9, 3, 2, 0, 1];
    let index = find_index(&arr, 2);

    println!("index for key 2 is {}", index);
}

fn find_index(arr: &[i32], key: i32) -> i32 {
    let mut index: usize = 0;
    let mut found_index: i32 = -1;
    while(index < arr.len()) {
        if(arr[index] == key) {
            found_index = index as i32;
            break;
        }
        index += 1;
    }

    found_index
}
```
*`break` and `continue` statements works in the similar manner to that of other languages.*

### `loop` loop

It is another type of loop construct available in rust which define infinite loops. It is controlled using `break` and `continue` statements within the loop.

In `loop` loop is also an expression and be used to return values using `break` statment as `break <value>`;

```rust
fn main() {
    let arr = [3, 9, 3, 2, 0, 1];
    let key = 77;
    let index = find_index(&arr, 77);

    println!("index for key {} is {}", key, index);
}

fn find_index(arr: &[i32], key: i32) -> i32 {
    let mut index: usize = 0;
    let found_index: i32 = loop {
        if(index ==  arr.len()) {
            break -1;
        }
        if(arr[index] == key) {
            break index as i32;
        }
        index += 1;
    };

    found_index
}
```