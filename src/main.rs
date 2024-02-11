fn main() {
    let one = Option::Some(1);
    let two = plus_one(one);
    let none = plus_one(Option::None);

    if let Some(n) = none {
        println!("{}", n);
    } else {
        panic!("no value found");
    }
}

fn plus_one(v: Option<i32>) -> Option<i32> {
    match v {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1)
    }
}