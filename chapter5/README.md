# 5. Using Structs to Structure Related Data

A *struct* or *structure*, is a custom data type that lets you package together and name multiple related values that maek up a meaningful group.

## 5.1 Defining and Instantiating Structs
Structs are similar to tuples:
- can hold multiple related values
- pieces of struct can be different types
But unlike tuple, in struct you'll name each piece of data so it's clear what the values mean. You don't have to rely on the order of the dat to specify or access the values of an instance.

To define struct, enter keyword `struct` and name the entire struct. Then inside curly brackets, define the names and types of the pieces of data, which we call `fields`.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

To use a struct after we've defined it, we create an *instance* of that struct by specifying concrete values for each of the fields.
To get specific value from a struct, we use dot notation.

```rust
fn main() {
    let user1 = User {
        active: true,
        username: String::from("somusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@example.com");
}
```

Note that the entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable.

### Using the Field Init Shorthand
If the parameter names and the struct field names are exactly the same, we can use the *field init shorthand* syntax.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

### Creating Instances with Struct Update Syntax
```rust
fn main() {
    // --snip--
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

Using struct update syntax, we can achieve the same effect with less code.
```rust
fn main() {
    // --snip--
    let user2 = User {
        email: String::from("another@example.com"),
       ..user1
   };
}
```

Note that the struct update syntax uses `=` like an assignment; this is because it moves the data. In this example, we can no longer use `user1` after creating `user2` because the `String` in the `username` field of `user1` was moved into `user2`. If we had given `user2` new `String` values for both `email` and `username`, and thus only used the `acitve` and `sign_}in_count` values from `user1`, then `user1` would still be valid after creating `user2`. Both `active` and `sign_in_count` are types that implement the `Copy` trait. We can also still use `user1.email` in this example, because its values was not moved out of `user1`.

### Creating Different Types with Tuple Structs
Tuple structs have the added meaning the struct name provides but don't have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Note that `black` and `origin` values are different types because they're instances of different tuple structs. Unlike tuples, tuple structs require you to name the type of the struct when you destructure them: `let Point(x, y, z) = origin;`.

### Defining Unit-Like Structs
You can also define structs that don't have any fields, called *unit-like structs* because they behave similarly to the unit type `()`. Unit-like structs can be useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.

```rust
struct AlwaysEqual;
fn main() {
    let subject = AlwaysEqual;
}
```

### Ownership of Struct Data
In the `User` struct definition, we used the owned `String` type rather than the `&str` string slice type. This is deliberate choice because we want each instance of this struct to own all of its data and for that data to be valid for as long as the enitre struct is valid.

It's also possible for structs to store references to data owned by something else, but to do so requires the use of *lifetime*. Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.

```rust
// This won't compile!
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
```
The compiler will complain that it needs lifetime specifiers.


## 5.2 An Example Program Using Structs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

### Refactoring with Tuples

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

```

### Refactoring with Structs
```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### Adding Functionality with Derived Traits

The `println!` macro can do many kinds of foramatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption. The primitive types implement `Display` by default.

Putting the specifier `:?` or `:#?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. Rust includes functionality to print out debugging information, but we have to explicitly opt into make that functionality available for our struct. To do that, add the outer attribute `#[derive(Debug)]` above the struct definition.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {rect1:?}");
}
```

Another way to print out a value using the `Debug` format is to use the `dbg!` macro, which takes ownership of an expression (as opposed to `println!`, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value. 
*Note: Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!`, which prints to the standard output console stream (`stdout`).*


```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

In addition to the `Debug` trait, Rust has provided a number of traits for us to use with the `derive` attribute that can add useful behaviour to our custom types.

## 5.3 Methods

Methods are similar to functions: declare them with the `fn` keyword and a name, can have parameters and a return value, contain some code that's run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or trait object), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

### Method Syntax

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // method syntax
    );
}
```

To define the function within the context of `Rectangle`, we start an `impl` block for `Rectangle`. In the signature for `area`, we use `&self` intead of `rectangle: &Rectangle`. The `&self` is actually a short for `self: &Self`. Within an `imple` block, the type `Self` is an alias for the type that the `impl` block is for. Methods must have a parameter named `self`of type `Self` for their first parameter, so Rust lets you abbreiviate this with only the name `self` in the first parameter spot. Methods can take ownership of `self`, borrow `self` immutable, as we've done here, or borrow `self` mutably. We chose `&self` here for the same reason we use `&Rectangle` in the function version: We don't want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we've called the method on as part of what the method does, we'd use `&mut self` as the first parameter. Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.

Note that we can choose to give a method the same name as one of the struct's fields.

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called *getters*, and Rust does not implement them automatically for struct fields. 

### Where's the -> Oprator?
In C and C++, two different operators are used for calling methods: You use `.` if you're calling a method on the object directly and `->` if you're calling the method on a pointer to the object and need to dereference the pointer first. In other words, if `object` is a pointer, `object->somthhing()` is similar to `(*object).somthing()`.
Rust doesn't have an equivalent to the `->` operator; insteand, Rust has a feature called *automatic referencing and dereferencing*. Calling methods is one of the few places in Rust with this behaviour.

When you call a method with `object.something()`, Rust automatically adds in `&`, `&mut`, or `*` so that `object` matches the signature of the method. In other words, the following are the same:
```
p1.distance(&p2);
(&p1).distance(&p2);
```

This automatic referencing behaviour works because methods have a clear receiver - the type of `self`. Given the receiver and name of a method, Rust can figure out definitvely whether the method is reading (`*self`), mutating (`&mut self`), or consuming (`self`). The fact that Rust makes borrowing implicit for method receivers is a big part of making ownership ergonomic in practice.


### Methods with More Parameters

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### Associated Functions
All functions defined within `impl` block are called *associated functions* because they're associated with the type named after the `impl`. We can defined associated functions that don't have `self` as their first parameter (and thus are not methods) because they don't need an instance of the type to work with. We've already used one function like this: the `String::from` function that's defined on the `String` type.

Associated functions that aren't methods are often used for constructors that will return a new instance of the struct. These are often called `new`, but `new` isn't a special name and isn't built into the language.

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

To call this associated function, we use `::` syntax with the struct name; `let sq = Rectange::square(3);`. This function is namespaced by the struct: The `::` syntax is used for both associated functions and namespaces created by modules.

### Multiple `impl` Blocks 
Each struct is allowed to have multiple `impl` blocks.
```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```


