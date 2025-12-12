fn main() {
    variables_and_mutability();
    data_types();
    functions();
    comments();
    control_flow();
}

// 3.1 Variables and Mutability
fn variables_and_mutability() {
    println!("============== 3.1 Variables and Mutability ==============");

    // Testing mutability of a variable
    println!("Testing mutability of a variable");
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}\n\n");

    // Declaring constants with data type
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of constant THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}\n\n");

    // Shawdowing
    let z = 5;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}\n\n");

    println!("Demonstrating ability to mutate the shadow variable's type");
    let spaces = "  ";
    println!("The value of spaces: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces: {spaces}");

    // let mut mut_spaces = "  ";
    // println!("The value of mut_spaces: {mut_spaces}");
    // let mut_spaces = mut_spaces.len(); // Expected to throw error

    println!("\n\n");
}

// 3.2 Data Types
fn data_types() {
    println!("============== 3.2 Data Types ==============");

    // Integer Types
    let x: i8 = -128;
    let y: u8 = 255;
    println!("This is an i8 variable x: {x} and a u8 variable y: {y}");

    let decimal_literal = 100_000;
    let decimal_literal_u8: u8 = 57u8;
    let hex_literal = 0xff;
    let octal_literal = 0o77;
    let binary_literal = 0b1111_0000;
    let byte_literal = b'A';

    println!("This is decimal literal 100_000: {decimal_literal}");
    println!("This is decimal literal 57 of u8 type: {decimal_literal_u8}");
    println!("This is hex literal 0xff: {hex_literal}");
    println!("This is octal literal 0o77: {octal_literal}");
    println!("This is binary literal 0b1111_0000: {binary_literal}");
    println!("This is byte literal b'A': {byte_literal}\n\n");

    // Integer Overflows
    let a: u8 = 255;
    let b: u8 = 1;

    // 1. wrapping_* methods
    // Wraps around.
    let wrapped_sum = a.wrapping_add(b);
    println!("1. Wrapping: 255 + 1 = {wrapped_sum}");

    // 2. checked_* methods
    // Returns an Option<T>. If no overflow, a value is returned. None if overflows.
    let checked_sum = a.checked_add(b);
    match checked_sum {
        Some(val) => println!("2. Checked: Sum is {val}"),
        None => println!("2. Checked: Overflow occurred, returned None"),
    }

    // 3. overflowing_* methods
    // Rerturns a tuple: (value, overflowed: bool). The value is the wrapped value.
    let (overflow_val, overflowed) = a.overflowing_add(b);
    println!("3. Overflowing: Value is {overflow_val}, Overflowed: {overflowed}");

    // 4. saturating_* methods
    // Clamps the result to the maximum or minimum value
    let saturated_sum = a.saturating_add(b);
    println!("4. Saturating: {a} + {b} = {saturated_sum}");

    // Floating-Point Types
    let _f = 2.0; // f64
    let _g: f32 = 3.0; // f32

    // Numeric Operations

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;

    // Boolean Type
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // The Character Type

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_t1, _t2, t3) = tup; // destructure a tuple to get individual values out
    println!("The value of t3 is: {t3}");
    println!("The third element of tuple is: {}\n\n", tup.2);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let _brr = [3; 5]; // [3, 3, 3, 3, 3];
    let _first = arr[0];
}

// 3.3 Functions
fn functions() {
    println!("============== 3.3 Functions ==============");

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is: {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }
    let z = plus_one(5);
    println!("The value of z is :{z}\n\n");
}

// 3.4 Comments
fn comments() {
    println!("============== 3.4 Comments ==============\n\n");
}

// 3.5 Control Flow
fn control_flow() {
    println!("============== 3.5 Control Flow ==============");

    // If Expressions
    println!("If expressions");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {number}\n");

    // Repetition with Loops
    println!("Repetition with loops");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}\n");


    // Loop labels
    println!("Loop labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remainig = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}\n");

    // Conditional loops with while
    println!("Conditional loops with while");

    let mut num = 3;
    while num != 0 {
        println!("{num}!");
        num -= 1;
    }

    println!("LIFTOFF!!!");

    // For loops
    println!("For loops");

    // Looping through each element of a collection using a while loop
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    // Looping through each element of a collection using a for loop
    for element in a {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
