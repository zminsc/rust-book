fn main() {
    // calling functions
    another_function();
    another_function_2(5);
    print_labeled_measurement(5, 'h');

    // statements: instructions that perform an action but DO NOT return a value
    let _y = 6;

    // expressions: instructions that evaluate to a resultant value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");
    let x = plus_one(x);
    println!("The value of x is now: {x}");
}

// functions can be defined below where they are used
fn another_function() {
    println!("Another function.");
}

// functions can have parameters
fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

// functions can have multiple, comma-separated parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// functions can have return values
fn five() -> i32 {
    5
}

// putting it all together
fn plus_one(x: i32) -> i32 {
    x + 1 // note: you cannot put a semicolon here, that'll make it a statement
}

// example "accidental" statement
fn _oops() -> () {
    5;
}
