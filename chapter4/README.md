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
- Variables live in the stack
    - Variables live in frames. A frame is a mapping from variables to values within a single scope, such as a function. Frames are organized into a stack of currently-called-functions
    - ```rust
        fn main() {
            let n = 5; // L1
            let y = plus_one(n); // L3
            println!("The value of y is: {y}");
        }

        fn plus_one(x: i32) -> i32 {
            x + 1 // L2
        }
      ```
      - The frame for `main` at `L1` holds `n=5`
      - The frame for `plus_one` at `L2` holds `x=5`
      - The frame for `main` at `L3` holds `n=5;y=6`
    - After a function returns, Rust deallocates the function's frame (freeing or dropping)
    - This sequence of frames is called a stack because the most recent frame added is always the next frame freed
    - When an expression reads a variable, the variable's value is copied from its slot in the stack frame
        - ```rust
          let a = 5; // L1
          let mut b = a; // L2
          b += 1; // L3
          ```
        - L1 Stack main |a|5|
        - L2 Stack main |a|5|
                        |b|5|
        - L3 Stack main |a|5|
                        |b|6|

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

```rust
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value,
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value

let r2: &i32 = &*x;      // r2 points to the heap value directly
let c: i32 = *r2;        // so only one dereference is needed to read it
```

You probably won't see the dereference operator very often when you read Rust code. Rust implicitly inserts dereferences and references in certain cases, such as calling a method with the dot operator.

```rust
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // explicit dereference
let x_abs2 = x.abs();      // implicit dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // explicit dereference (twice)
let r_abs2 = r.abs();       // implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // explicit reference
let s_len2 = s.len();      // implicit reference
assert_eq!(s_len1, s_len2);
```

Pointers are a powerful and dangerous feature because they enable aliasing. Aliasing is accessing the same data through different variables. On its own, aliasing is harmlss. But combined with mutation, we have a recipe for disaster.
- by deallocating the aliased data, leaving the other variable to point to deallocated memory
- by mutating the aliased data, invalidating runtime properties expected by the other variable
- by concurrently mutating the aliased data, causing a data race with nondeterministic behaviour for the other variable


Example program using vector data structer, `vec`. Unlike arrays which have a fixed length, vectors have a variable length by storing their elements in the heap.

```rust
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // macro vec! creates vector with the elements between the brackets
    v.push(4);
}
```

`v` allocates a heap array of certain *capacity*. If the vector is at capacity. To allow `push`, it has to create a new allocation with larger capacity, copy all the elements over, and deallocate the original heap array. In the above example, the array `1 2 3 4` is in a (potentially) different memory location than the original array `1 2 3`.

```rust
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2]; // L1
    v.push(4); // L2
    println!("Third element is {}", *num); // L3
}
```

The operation `v.push(4)` resizes `v`. The resize will deallocate the previous array and allocate a new bigger array. In the process, `num` is left pointing to invalid memory. Therefore at L3, dereferencing `*num` reads invalid memory, causing undefined behaviour.
In more abstract terms, the issue is that the vector `v` is both aliased (by the reference `num`) and mutated (by the operation `v.push(4)`).

Pointer Safety Principle: data should never be aliased and mutated at the same time.
Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated. For example, Rust enforces this principle for boxes (owned pointers) by disallowing aliasing. Assigning a box from one variable to another will move ownership, invalidating the previous variable. Owned data can only be accessed through the owner - no aliases.
However, because references are non-owning pointers, they need different rules than boxes to ensure the Pointer Safety Principle.

The core idea behind the borrow checker is that variables have three kinds of permissions on their data:
- Read: data can be copied to another location
- Write: data can be mutated
- Owned: data can be moved or dropped

These permissions don't exist at runtime, only within the compiler. By default, a variable has read/own permissions on its data. If a variable is annotated with `let mut`, then it also has the write permission. The key idea is that references can temporarily remove these permissions.


Let’s walk through each line:

1. After `let mut v = (...)`, the variable `v` has been initialized. It gains +R+W+O permissions 
2. After `let num = &v[2]`, the data in `v` has been borrowed by `num`. Three things happen:
   - The borrow removes WO permissions from `v`. `v` cannot be written or owned, but it can still be read
   - The variable `num` has gained RO permissions. `num` is not writable because it was not marked `let mut`
   - The place `*num` has gained the R permission 
3. After `println!(...)`, then `num` is no longer in use, so `v` is no longer borrowed. Therefore:
   - `v` regains its WO permissions
   - `num` and `*num` have lost all of their permissions
4. After `v.push(4)`, then `v` is no longer in use, and it loses all of its permissions

Permissions are defined on **places** and not just variables. A place is anything you can put on the lef-hand side of an assignment. Places include:
- variables, like `a`
- dereferences of places, like `*a`
- array accesses of places, like `a[0]`
- fields of places, like `a.0` for uples or `a.field for structs
- any combination of the above, like `*((*a)[0].1)`

Creating a reference to data ("borrowing" it) causes that data to be temporarily read-only until the reference is no longer in use.
Rust uses these permissions in its *borrow checker* The borrow checker looks for potentially unsafe operations involving references.

If you try to compile this program, then the Rust compiler will return the following error:
```shell
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> test.rs:4:1
  |
3 | let num: &i32 = &v[2];
  |                  - immutable borrow occurs here
4 | v.push(4);
  | ^^^^^^^^^ mutable borrow occurs here
5 | println!("Third element is {}", *num);
  |                                 ---- immutable borrow later used here
```

The error message explains that `v` cannot be mutated while the reference `num` is in use. That's the surface-level reason. The underlying issus that `num` could be invalidated by `push`. Rust catches that potential violation of memory safety.


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



```rust
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}
```
1. When `num` was an immutable reference, `v` still had R permission. Now that `num` is a mutable reference,`v` has lot all permissions while `num` is in use
2. When `num` was an immutable reference, the place `*num` only had R permission. Now that `num` is a mutalbe reference, `*num` has also gained the W permission

Mutable reference can also be temporarily "downgraded" to read-only references

```rust
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = &*num;
    println!("{} {}", *num, *num2);
}
```
The borrow `&*num` removes the W permission from `*num` but not the R permission. So `println!(..)` can read both `*num` and `*num2`. 

The phrase "in use" is describing a reference's **lifetime**, or the range of code spanning from its birth (where the reference is created) to its death (the last time(s) the reference is used).

```rust
fn main() {
    let mut x = 1;
    let y = &x; // life time of y starts here
    let z = *y; // life time of y ends here, and the W permission on x is returned to x
    x += z;
}
```

```rust
fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
```
The variable `c` has a different lifetime in each branch of the if-statement. In the then-block, `c` is used in the expression `c.to_ascii_uppercase()`. Therefore `*v` does not regain the W permission until after that line. Howver, in the else-block, `c` is not used. `*v` immediately regains the W permission on entry to the else-block.


As a part of the Pointer Safety Principle, the borrow checker enforces that data must outlive any references to it. Rust enforces this property in two ways. The first way deals with references that are created and dropped within the scope of a single function. For example, say we tried to drop a string while holding a reference to it:
```rust
fn main() {
    let s = String::from("Hello world");
    let s_ref = &s;
    drop(s);
    println!("{}", s_ref);
}
```
The borrow `&s` removes the O permission from `s`. However, `drop` expects the O permission, leading ot a permission mismatch.

But Rust needs a different enforcement mechanism when it doesn't know how long a reference lives. Specifically, when references are either input to a function, or output from a function. For example, here is a safe function that returns a reference to the first element in a vector:
```rust
fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    s_ref
}
```

This snippet introduces a new find of permission, the flow permission F. The F permission is expected whenever an expression uses an input reference like `&strings[0]`, or returns an output reference like `return s_ref`.

Unlike RWO permissions, F does not change throughout the body of a function. A reference has F permission if it's allowed to be used (that is, to flow) in a particular expression.

```rust
fn first_or<'a, 'b, 'c>(strings: &'a Vec<String>, default: &'b String) -> &'c String {
    if strings.len() > 0 {
        &strings[0]
    } else {
        default
    }
}
```

This function no longer compiles, because the expression `&string[0]` and `default` lack the necessary F permission to be returned. If Rust just looks at the function signature, it doesn't know whether the output `&String` is a reference to either `strings` or `default`.
The following program is unsafe if `first_or` allows `default` to flow into the return value. Like the previous example, `drop` could invalidate `s`.
```rust
fn main() {
    let strings = vec![];
    let default = String::from("default");
    let s = first_or(&strings, &default);
    drop(default);
    println!("{}", s);
}
```

```rust
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    let s_ref = &s;
    s_ref
}
```
This program is also unsafe because the reference `&s` will be invalidated when `return_a_string` returns.

#### Fixing Ownership Errors

Rust will always reject un unsafe program. But sometimes, Rust will also reject a safe program. 

##### Fixing an Unsafe Program: Returning Reference to the Stack

```rust
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```

The issue here is with the lifetime of the referred data. If you want to pass around a reference to a string, you have to make sure that the underlying string lives long enough. When the function returns, `s` is dropped. The code attempts to return `&s` but that no longer extsts after the funciton returns. Any pointer/reference to that memory would be a dangling reference. Dereferencing it would be undefined behaviour.

Depending on the situation, here are four ways you can extend the lifetime of the string. 

1. Move ownership of the string out of the function, changing `&string` to `String`:

```rust
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
```

2. Return a string literal, which lives forever (indicated by `'static`). This applies if we never intend to change the string, and then a heap allocation is unnecessary:
```rust
fn return_a_string() -> &'static str {
    "Hello world"    
}
```

3. Defer borrow-checking to runtime by using garbage collection. For example, you can use a reference-counted pointer:
```rust
use std::rc::Rc;
fn return_a_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
```
`Rc::clone` only clones a pointer to `s` and not the data itself. At runtime, the `Rc` checks when the last `Rc` pointing to data has been dropped, and then deallocates the data.

4. Have the caller provide a "slot" to put the string using a mutable reference:
```rust
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}
```

With this strategy the caller is responsible for creating space for the string. This style can be verbose, but it can also be more memmory-efficient if the caller needs to carefuly control when allocation occur.


##### Fixing an Unsafe Program: Not Enough Permission
Another common issue is trying to mutate read-only data, or trying to drop data behind a reference.

```rust
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
```
This program is rejected by the borrow checker because `name` is an immutable reference, but `name.push(..)` requires the W permission. This program is unsafe because `push` could invalidate other references to `name` outside of `stringify_name_with_title`, like this:


```rust
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}
```

In this example, a reference `first` to `name[0]` is created before calling `stringify_name_with_title`. The function `name.push(..)` reallocates the contents of `name`, which invalidates `first`, causing the `println` to read deallocated memory.

1. Change the type of name from `&Vec<String>` to `&mut Vec<String>`:
```rust
fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

But this is not a good solution if the caller is not expecting the function to mutate their inputs. 

2. Take ownership of the name by changing `&Vec<String>` to `Vec<String>`:
```rust
fn stringify_name_with_title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

But it is very rare for Rust functions to take ownership of heap-owning dat structures like `Vec` and `String`. This would make the input `name` unusuable.

3. Change the body of the function to clone the input `name`:
```rust
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
```
By cloning, we are allowed to mutate the local copy of the vector. However, the clone copies every string in the input. We can avoid unnecessary copies by adding the suffix later:
```rust
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
```
This solution works because `slice::join` already copies the data in `name` into the string `full`.            


##### Fixing an Unsafe Program: Aliasing and Mutating a Data Structure

Another unsafe operation is using a reference to heap data that gets deallocated by another alias. For example, here's a function that gets a reference to the largest string in a vector, and then uses it while mutation the vector:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```
This program is rejected by the borrow checker because `let largets = ..` removes the W permissions on dst. However, `dst.push(..)` requires the W permission. The program is unsafe because `dst.push(..)` could deallocate the contents of `dst`, invalidating the reference `largest`.

We need to shorten lifetime of `largest` to not overlap with `dst.push(..)`. One possibility is to clone `largest`:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```

However, this may cause a performance hit for allocating and copying the string data.

Another possiblity is to perform the length comparison first, and then mutate `dst` afterwords:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = 
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}
```
However, this also causes a performance hit for allocating the vector `to_add`.

A final possiblilty is to copy out the length of `largest`, since we don't actually need the contents of `largest`, just its length. This solution is arguably the most idiomatic adn most performant:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
```


##### Fixing an Unsafe Program: Copying vs Moving Out of a Collection
Here's a safe program that copies a number out of a vector:

```rust
fn main() {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    let n: i32 = *n_ref;
}
```

The dereference operation `*n_ref` expects just the R permission, which the path `*n_ref` has. But what happens if we change the type of elements in the vector from `i32` to `String`? Then it turns out we no longer have the necessary permissions:

```rust
fn main() {
    let v: Vec<String> = 
      vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    let s: String = *s_ref;
}
```

The issue is that the vector `v` owns the string "Hello world". When we dereference `s_ref`, that tries to take owenership of the string from the vector. But references are non-owning pointers - we can't take ownership through a reference. The program will be unsafe due to double-freeing. After `s` is dropped, "Hello world" is deallocated. Then `v` is dropped, and undefined behaviour happens when the string is freed a second time. However, this undefined behaviour does not happen when the vector contains `i32` elements. The difference is that copying `String` copies a pointer to heap data. Copying an `i32` does not. If a value does not own heap data, then it can be copied without a move. 
- `i32` does not own heap data, so it can be copied without a move
- `String` does own heap data, so it can not be copied without a move
- `&String` does not own heap data, so it can be copied without a move

*Note: one exception to this rule is mutable reference. For example, `&mut i32` is not a copyable type.

```rust
let mut n = 0;
let a = &mut n;
let b = a;
```

Then `a` cannot be used after being assigned to `b`. That prevents two mutable reference to the same data from being used at the same time.

To safely access an element in vector of non-`Copy` types like `String`:

1. Avoid taking ownership of the string and just use an immutable reference:

```rust
let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = &v[0];
println!("{s_ref}!");
```

2. Clone the data if you want to get ownership of the string while leaving the vector alone:

```rust
let v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v[0].clone();
s.push('!');
println!("{s}");
```

3. Use a method like `Vec::remove` to move the string out of the vector:
```rust
let mut v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v.remove(0);
s.push('!');
println!("{s}");
assert!(v.len() == 0);
```

##### Fixing a Safe Program: Mutating Different Tuple Fields
```rust
fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = &name.0;
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
```

The statement `let first = &name.0` borrows `name.0`. This borrow removes WO permissions from `name.0`. It also removes WO permissions from `name`. But `name.1` still retains the W permission, so doing `name1.push_str(..)` is a valid operation.

However, Rust can lose track of exactly which places are borrowed. For example, let's say we refactor the expression `&name.0` into a function `get_first`. Ater calling `get_first(&name)`, Rust now removews the W permission on `name.1`.

```rust
fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = get_first(&name);
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}
```

Now we can't do `name.1.push_str(..)`. The problem is that Rust doesn't look at the implementation of `get_first` when deciding what `get_first(&name)` should borrow. Rust only looks at the type signature, which just says "some `String` in the input gets borrowed". Rust conservatively decides then that both `name.0` and `name.1` get borrowed, and eliminates write and own permissions on both.


##### Fixing a Safe Program: Mutating Different Array Elements

A similar kind of problem arises when we borrow elements of an array. 
```rust
fn main() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    *x += 1;
    println!("{a:?}");
}
```

Rust's borrow checker does not contain different places for `a[0]`, `a[1]`, and so on. It uses a single place `a[_]` that represents all indices of `a`. Rust does this because it cannot always determine the value of an index. 

```rust
fn main() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
}
```

Rust will reject this program because `a` gave its read permission to `x`. This program is safe. Rust provides a function in the standard library that can work around the borrow checker:

```rust
let mut a = [0, 1, 2, 3];
let (a_l, a_r) = a.split_at_mut(2);
let x = &mut a_l[1];
let y = &a_r[0];
*x += *y;
```

In some Rust libraries, especially core types like `Vec` or `slice`, you will often find `unsafe` blocks. `unsafe` block allow the use of "raw" pointers, which are not checked for safety by the borrow checker. For example, we could use an unsafe block to accomplish our task:

```rust
let mut a = [0, 1, 2, 3];
let x = &mut a[1] as *mut i32;
let y = &a[2] as *const i32;
unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
```

Unsafe code is sometimes ncessary to work around the limitations of the borrow checker. As a general strategy, let's say the borrow checker rejects a program you think is actually safe. Then you should look for standard library functions (like split_at_mut) that contain unsafe blocks which solve your problem.
 
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













