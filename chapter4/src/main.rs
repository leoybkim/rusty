fn main() {
    ownership();
    reference_and_borrowing();
    slice_type();
}

// 4.1 What is Ownership
fn ownership() {
    println!("============ 4.1 What is Ownership =============");
    // The String Type
    // This kind of string can be mutated
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let s2 = s;
    // println!("{s}"); // Should throw error here because s was moved to s2
    println!("s2 = {s2}");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}\n\n");

    some_scope();
    another_scope();

    let hello = String::from("hello");
    let (test, len) = calculate_length(hello);
    println!("The length of '{test}' is {len}.");
}

fn reference_and_borrowing() {
    println!("============ 4.1 What is Ownership =============");
}

fn slice_type() {
    println!("============ 4.1 What is Ownership =============");
}

fn some_scope() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5;          // x comes into scope

    makes_copy(x);      // Because i32 implements the Copy trait,
                        // x does NOT move into the function,
                        // so it's okay to use x afterward.
} // Here, my_number goes out of scope, then my_string. However, because my_string's value was
  // moved, nothing special happens.

fn another_scope() {
    let s1 = gives_ownership();     // gives_ownership moves its return
                                    // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
