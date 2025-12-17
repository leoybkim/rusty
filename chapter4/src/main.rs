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
    println!("============ 4.2 References and Borrowing =============");

    let s1 = String::from("hello");
    let len = calculate_length_ref(&s1);
    println!("The length of '{s1}' is {len}\n\n");

    // Mutable reference
    let mut s = String::from("hello");
    change(&mut s);
    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1} and {r2}"); // This will throw error at compile time because you cannot
    // borrow `s` as mutable more  than once at a time
    println!("{r1}");

    // Multiple mutable reference in a separate scope
    multi_ref_separate_scope();

    // Cannot combine mutable and immutable reference simultaneously
    let mut ss = String::from("hello");

    let r3 = &ss; // no problem
    let r4 = &ss; // no problem
    // let r5 = &mut ss; // BIG PROBLEM

    println!("{r3} and {r4}");
    // Variables r3 and r4 will not be used after this point.

    let r5 = &mut ss; // no problem
    println!("{r5}");

    let reference_s = no_dangle();
    println!("{reference_s}");

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4);
    // println!("Third element is {}", *num); // error!
    

    let mut char_v: Vec<char> = vec!['a', 'b', 'c'];
    ascii_capitalize(&mut char_v);


    let mut strs = vec![
        String::from("A"), String::from("B")
    ];

    let first = get_first(&strs);
    if first.len() > 0 {
        strs.push(String::from("C"));
    }
    println!("strs: {:?}", strs);
}

fn slice_type() {
    println!("============ 4.3 The Slice Type =============");
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear(); // will error
    println!("{word}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word(my_string_literal);

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
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
    println!("s1: {s1}, s3: {s3}, s2 was moved so we cannot print it");
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

fn calculate_length_ref(s: &String) -> usize {
    // s.push_str(", world"); // This will throw error because you are not allowed to modify a borrowed value
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change(some_string: &mut String) {
    // can edit mutable reference
    some_string.push_str(", world");
}

fn multi_ref_separate_scope() {
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // _r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{r2}");
}

/*
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!
*/

fn no_dangle() ->String{
    let s = String::from("hello");
    s
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
        println!("Capitalized: {:?}", v);
    } else {
        println!("Already capitalized: {:?}", v);
    }
}

fn get_first(v: &Vec<String>) -> &String {
    &v[0]
}
