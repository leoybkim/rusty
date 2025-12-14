# 4. Understanding Ownership

Ownership enables Rust to make memory safety guarantees without needing a garbage collector.

## 4.1 What Is Ownership
*Ownership* is a set of rules that govern how a Rust program manages memory. Other languages have garbage collection or have the programmer explicitly allocate and free the memory. Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile. None of the features of ownership will slow down your program while it's running.

### The Stack and the Heap
In systems programming language like Rust, whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions.
Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.

**Stack**:
- stores value in the order it gets them and removes th evaluates in the opposite order (LIFO)
- all data stored on the stack must have a known, fixed size

**Heap**:
- request a certain amount of space to put data on heap
    - memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of that location
        - this process is called *allocating on the heap* or just *allocating*
        - the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer

Pushing to the stack is faster than allocating, as well, accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.

When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaining up unused data on the heap so that you don't run out of space are all problems that ownership addresses.


### Ownership Rules
- Each value in Rust has an *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be dropped

### Variable Scope
A *scope* is the range within a program for which an item is valid.

### The `String` Type
The types covered in [Chapter 3](../chapter3/README.md) are of a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope. But `String` is a data type that is stored on the heap. This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. You can create a `String` from a string literal using the `from` function.
```rust
let s = String::from("hello");
```
The double colon `::` operator allows us to namespace this particular `from` function under `String` type.
This kind of string *can* be mutated:
```rust
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{s}");
```

### Memory and Allocation
In the case of string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This makes string literal fast and efficient, but immutable. With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.
- The memory must be requested from the memory allocator at runtime
- We need a way of returning this memory to the allocator when we're done with our `String`

The first part is already done when we call `String::from`, its implementation request the memory it needs, which is pretty much universal in programming languages. The second part is different. In languages with a *garbage colector (GC)*, the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it. In languages without GC, it's our responsibility to identify when memory is not longer being used and to call code to explicitly free it. Doing this correctly has historically been a difficult programming problem. If we forget, we'll waste memory. If we do it too early, we'll have an invalid variable. If we do it twice, that's a bug too. We need to pair exactly one `allocate` with exactly one `free`. 

Rust take a different path: The memory is automatically returned once the variable that owns it goes out of scope. 
```rust
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s 
}                                  // this scope is now over, and s is no longer valid
```

When a variable goes out of scope, Rust calls a special function for us called `drop`. Rust calls `drop` automatically at the closing curly bracket. *(Note: In C++, this pattern of deallocating resources at the end of an item's lifetime is sometimes called Resource Acquisition Is Initialization (RAII))*

#### Variables and Data Interacting with Move
```rust
let x = 5;
let y = x;
```
In this example, we now have two variables, `x` and `y`, and both equal `5`. Because integers are simple values with a known, fixed size, these two `5` values are pushed onto the stack.
```rust
let s1 = String::from("hello");
let s2 = s1;
```
A `String` is made up of three parts:
- pointer to the memory that holds the contents of the string
- length -> how much memory, in bytes, the contents of the `String` are currently using
- capacity -> total amount of memory, in bytes, that the `String` has received from the allocator

This group of data is stored on the stack.
When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the pointer, the length and the capacity that are on the stack. We do not copy the data on the heap that the pointer referes to. When `s2` and `s1` go out of scope, they will both try to free the same memory, known as *a double free* error and is one of the memory safety bugs. Freeming memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities. To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes out of scope. Rust invalidating the first variable is known as the *move*.

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{s1}, world!"); // This line will throw error 'borrow of moved value: `s1`'
```

Rust will never automatically create "deep" copies of your data. Therefore, any *automatic* copying can be assumed to be inexpensive in terms of runtime performance.

#### Scope and Assignment
The inverser of this is true for the relationship between scoping, ownership, and memory being freed via the `drop` function as well. When you assign a completly new value to an existing variable, Rust will call `drop` and free the original value's memory immediately.

```rust
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");
```

#### Variables and Data Interacting with Clone
If we *do* want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

#### Stack-Only Data: Copy
Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are. If a type implements the `Copy` trait, variables that use it do not move, but rather are tivially copied, making that still valid after assignment to another variable.

Rust won't let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.

As a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`.
- All the integer types, such as `u32`
- The Boolean type, `bool`, with values `true` and `false`
- All the floating-point types, such as `f64`
- The character type `char`
- Tuples, if they only contain types that also implement `Copy`

### Ownership and Functions
Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // Because i32 implements the Copy trait,
                                    // x does NOT move into the function,
                                    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
```

### Return Values and Scope
Returning values can also transfer ownership. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data base been moved to another variable.

```rust
fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
                                       // return value into the function
                                       // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                        // some_string is returned and
                                       // moves out to the calling
                                       // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
## 4.2 References and Borrowing

A reference is like a pointer in that it's an address we can follow to access the data stored at that address; that data is owned by some other variable. Unliked a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // Here, s goes out of scope. But because s does not have ownership of what
   // it refers to, the String is not dropped.
```

The ampersands represent references, and they allow you to refer to some value without taking ownership of it.

*Note: The opposite of referencing by using `&` is *dereferencing*, which is accomplished with the dereference operator, `*`.*

We call the action of creating a reference *borrowing*. Just as variables are immutable by default, so are references. We're not allowed to modify something we have a reference to.

### Mutable References
You can allow modification of a borrowed value with *mutable reference* created with `&mut`. If you have a mutable reference to a value, you can have no other references to that value.
The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion. The benefit of having this restriction is that Rust can prevent data races at compile time. A *data race* is similar to a race condition and happens when these three behaviours occur:
- 2 or more pointers access the same data at the same time
- at least one of the pointers is being used to write to the data
- there's no mechanism being used to synchronize access to the data
Data races cause undefined behaviour and can be difficult to diagnose and fix when you're trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not *simultaneous* ones:
```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference

let r2 = &mut s;
```

Rust enforces a similar rule for combining mutable and immutable references. We cannot have a mutable reference while we have an immutable one to the same value.
However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data.

Note that the reference's scope starts from where it was introduced and continues through the last time the reference is used. For instance, this code will compile because the last usage of the immutable reference is in the `println!`, before the mutable reference is introduced:



```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM

println!("{r1} and {r2}");
// Variables r1 and r2 will not be used after this point.

let r3 = &mut s; // no problem
println!("{r3}");
```

### Dangling References

In languages with pointers, it's easy to erroneously create a *dangling pointer* - a pointer that references a location in memory that may have been given to someone else - by freeing some memory while preserving a pointer to that  memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.


```rust
// This code does not compile!

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {  // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be deallocated. But when we tried to return a reference to it, this reference would be pointing to an invalid `String`.

Instead, return the `String` directly:
```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

Ownership is moved out, and nothing is deallocated.


### The Rules of References
- At any given time, you can have *either* one mutable reference *or* any number of immutable references
- References must always be valid



## 4.3 The Slice Type
*Slices* let you reference a continguous sequence of elements in a collection. A slice is kind of reference, so it does not have ownersip.

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

```

We can convert `String` to an array of bytes using the `as_bytes` method. We can also create an iterator over the array of bytes using the `iter` method, which returns each element in a collection. `enumerate` method wraps the result of `iter` and returns each element as part of a tuple instead. The first element of the tuple is the index, the second is a reference to the element.


After finding out the index, we return `usize` on its own, but it's only a meaningful number in the context of the `&String`. There's no guarantee that the `String` will sill be valid in the future.

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); //  this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we 
    // could meaningfully use with the value 5, so word is now totally invalid!
}
```
The above code is logically incorrect but would not receive any errors at compile time.


### String Slices
A *string slice* is a reference to a contiguous sequence of the elements of a `String`, and it looks like this:
```rust
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
```

We create slices using a range within square brackets by specifying `[starting_index..ending_index]`, where `starting_index` is the first position in the slice and `ending_index` is one more than the last position in the slice. Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to `ending_index` minus `starting_index`.

With Rust's `..` range syntax, if you want to start at index 0, you can drop the value before the two periods: `&s[..5]`, if you want to end at last index, you can drop the trailling number: `[6..]`. You can also drop both values to take a slice of the entire string: `&s[..]`. 

*Note: String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.*

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

Now we have a straightforward API that's much hard to mess up because the compiler will ensure that the reference into the `String` remain valid. The compiler will let us much sooner that we have a problem with our code:

```rust
// This code does not compile!

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // error!
    println!("the first word is: {word}");
}
```

Because `clear` needs to truncate the `String`, it needs to get a mutable reference. The `println!` after the call to `clear` uses the reference in `word`, so the immutable reference must still be active at that point. Rust disallows the mutable reference in `clear` and the immutable reference in `word` from existing at the same time, and compilation fails.



#### String Literals as Slices

`let s = "Hello, world!;`

String literals are stored inside the binary. The type of `s` here is `&str`: it's a slice pointing to that specific point of the binary. This is also why string literals are immutable; `&str` is an immutable reference.

#### String Slices as Parameters

We can improve the `first_word` function by using a string slice for the type of the `s` parameter: `fn first_word(s: &str) -> str {`. This makes our API more general and useful without losing any functionality.

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```













