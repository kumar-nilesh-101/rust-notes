fn main() {
    let (sum, product) = sum_mult((2, 3));
    println!("{} {}", sum, product);
}

fn sum_mult(v: (i32, i32)) -> (i32, i32) {
    (v.0 + v.1, v.0 * v.1)
}