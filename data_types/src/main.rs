use std::io;

fn main() {
    // Simple types

    // Integers
    let _a: i32 = -5; // signed
    let _b: u32 = 10; // unsigned

    // Floating-point numbers
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Booleans
    let _t = true;
    let _f: bool = false;

    // Characters
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Numeric operations
    let _sum = 5 + 10;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 30;
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;
    let _remainder = 43 % 5;

    // Compound types

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    let elt = tup.0;
    println!("The 1st element of tup is: {elt}");

    // Arrays
    let _a = [1, 2, 3, 4, 5]; // data allocated on stack
    let _months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "July", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]; // fixed size
    let _b: [i32; 7] = [0, 1, 2, 3, 5, 8, 13]; // type annotations
    let c = [3; 5]; // c = [3, 3, 3, 3, 3];
    let _first = c[0];
    let _second = c[1];

    // Array index out of bounds
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
