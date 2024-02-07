fn main() {
    let x = {
        let y = 2;
        let z = 3;
        y * z // notice the absent ';', this implicitly returns the produced value.
    };

    let y = {
        let x = 2;
        let z = 3;
        x * z; // this causes nothing to be returned since the last statement ends with a ';'.
    }

    println!("{} {}", x, y);
}