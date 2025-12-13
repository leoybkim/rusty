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
}                                  // this scope is not over, and s is no longer valid
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
