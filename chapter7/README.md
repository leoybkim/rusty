# 7 Package, Crates and Modules

As a project grows, you should organize code by splitting it into multiple modules and then multiple files. A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies. For very large projects comprising a set of interrelated packages that evolve together, Cargo provides workspaces.

Rust module system include:
- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library of executable
- **Modules and use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

## 7.1 Packages and Crates
A *crate* is the smallest amount of code that the Rust compiler considers at a time. Even if you run `rustc` rather than `cargo` and pass a single source code file, the compiler considers that file to be a crate. Crates can contain modules, and the modules may be defined in other files that get compiled with that crate.

A crate can come in one or two forms: a binary crate or a library crate. *Binary crates* are programs you can compile to an executable that you can run, such as a command line program or a server. Each must have a function called `main` that defines what happens when executables runs. All the crates we've created so far have been binary crates.

*Library crates* don't have a `main` function, and they don't compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the `rand` crate provides functionality to generate randome numbers. Most of the time when Rustaceans say "create", the mean library crate, and they use "crate" interchangeably with the general programming concept of a "library".

The *crate root* is a source file that the Rust compiler starts from and  makes up the root module of your crate.

A *package* is a bundle of one or more crates that provides a set of functionality. A package contains a *Cargo.toml* file that describes how to build those crates. Cargo is actually a package that contains the binary crate for the command line tool you've been using to build your code. The Cargo package also conains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command line tool uses.

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that's a library or binary crate.

```shell
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Cargo follows a convention that *src/main.rs* is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains *src/lib.rs*, the package contains a library crate with the same name as the package, and *src/lib.rs* is its crate root. Cargo passes the crate root files to `rustc` to build the library or binary.

Here, we have a package that only contains *src/main.rs*, meaning it only contains a binary crate name `my-project`. If a package contains *src/main.rs* and *src/lib.rs*, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the *src/bin* directory: Each file will be a separate binary crate.

## 7.2 Control Scope and Privacy with Modules

### Modules Cheat Sheet

- **Start from the crate root**: when compiling a crate, the compiler first looks in the crate root file (usually *src/lib.rs* for a library crate and *src/main.rs* for a binary crate) for code to compile.
- **Declaring modules**: in the crate root file, you can declare new modules; say you declare a "garden" module with `mod garden;`. The compiler will look for the module's code in these places:
    - inline, within curly brackets that replace the semicolon following `mod garden`
    - in the file *src/garden.rs*
    - in the file *src/garden/mod.rs*
- **Declaring submodules**: in any file other than the crate root, you can declare submodules. For example, you might declare `mod vegetables;` in *src/garden.rs*. The compiler will look for the submodule's code within the directory named for the parent module in the places:
    - inline, directly following `mod vegetables`, within curly brackets instead of the semicolon
    - in the file *src/garden/vegetables.rs*
    - in the file *src/garden/vegetables/mod.rs*
- **Paths to code in modules**: once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an `Asparagus` type in the garden vegetables module would be found at `crate::garden::vegetables::Asparagus`.
- **Private vs. public**: code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
- **the `use` keyword**: within a scope, the `use` keyword creates shortcuts to items to reduce repetitioin of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`, you can create a shortcut with `use crate::garden::vegetables::Asparagus;`, and from then on you only need to write `Asparagus` to make use of that type in the scope.

Here, we create a binary crate named `backyard` that illustrates these rules. The crate's directory, also named *backyard*, contains these files adn directories:

```
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

The crate root file in this case is *src/main.rs*, and it contains:

Filename: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```

The `pub mod garden;` line tells the compiler to include the code it finds in *src/garden.rs*, which is:

Filename: src/garden.rs

```rust
pub mod vegetables;
```

Here, `pub mod vegetables;` means the code in *src/garden/vegetables.rs* is included too. That code is:

Filename: src/garden/vegetables.rs

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

### Grouping Related Code in Modules
*Modules* let us organize code within a crate for readability and easy reuse. Modules also allow us to control the *privacy* of items because code within a module is private by default. Private items are internal implementation details not available for outside use. We can choose to make modules and the items within them public, which exposes them to allow external code to use and depend on them.


Filename: src/lib.rs
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

By using modules, we can group related definitions together and name why they're related. Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them. 

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

The entire module true is rooted under the implicit module named `crate`. The tree also shows that some modules are siblings, meaning they’re defined in the same module; `hosting` and `serving` are siblings defined within `front_of_house`. If module A is contained inside module B, we say that module A is the *child* of module B and that module B is the *parent* of module.

## 7.3 Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same way we use a path when navigating a filesystem.

A path can take 2 forms:
- an *absolute path* is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal `crate`.
- a *relative path* starts from the current module and uses `self`, `super` or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

Filename: src/lib.rs
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

The `eat_at_restaurant` function is part of our library crate's public API, so we mark it with the `pub` keyword. The first time we call the `add_to_waitlist` function in `eat_at_restaurant`, we use an absolute path. The `add_to_waitlist` function is defined in the same crate as `eat_at_restaurant`, which means we can use the `crate` keyword to start an absolute path. We then include each of the successive modules until we make our way to add_to_waitlist. You can imagine a filesystem with the same structure: We’d specify the path `/front_of_house/hosting/add_to_waitlist` to run the `add_to_waitlist` program; using the `crate` name to start from the crate root is like using `/` to start from the filesystem root in your shell.

The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a relative path. The path starts with `front_of_house`, the name of the module defined at the same level of the module tree as `eat_at_restaurant`. Here the filesystem equivalent would be using the path `front_of_house/hosting/add_to_waitlist`. Starting with a module name means that the path is relative.

Choosing whether to use a relative or absolute path is a decision you’ll make based on your project, and it depends on whether you’re more likely to move item definition code separately from or together with the code that uses the item. Our preference in general is to specify absolute paths because it’s more likely we’ll want to move code definitions and item calls independently of each other.

The above code will fail to compile with error "module `hosting` is private". In other words, we have the correct paths for the `hosting` module and the `add_to_waitlist` function, but Rust won't let us use them because it doesn't have access to the private sections. In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. If you want to make an item like a function or struct private, you put it in a module.

Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.

Rust chose to have the module system function this way so that hiding inner implementation details is the default. That way, you know which parts of the inner code you can change without breaking the outer code. However, Rust does give you the option to expose inner parts of child modules’ code to outer ancestor modules by using the pub keyword to make an item public.

### Exposing Paths with the `pub` keyword
Filename: src/lib.rs
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}

// -- snip --
```

This code will still throw compile error: "function `add_to_waitlist` is private". Adding the `pub` keyword in front of `mod hosting` makes the module public. With this change, if we can access `front_of_house`, we can access `hosting`. But the contents of `hosting` are still private; making the module public doesn’t make its contents public. The `pub` keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.

Filename: src/lib.rs
```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// -- snip --

```


### Best Practices for Packages with a Binary and a Library

We mentioned that a package can contain both a *src/main.rs* binary crate root as well as a *src/lib.rs* library crate root, and both crates will have the package name by default. Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code defined in the library crate. This lets other projects benefit from the most functionality that the package provides because the library crate’s code can be shared.

The module tree should be defined in *src/lib.rs*. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: It can only use the public API. This helps you design a good API; not only are you the author, but you’re also a client!


### Starting Relative Paths with `super`


We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super` at the start of the path. This is like starting a filesystem path with the `..` syntax that means to go to the parent directory.

Filename: src/lib.rs

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

If we think the `back_of_house` module and the `deliver_order` function are likely to stay in the same relationship to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used `super` so that we’ll have fewer places to update code in the future if this code gets moved to a different module.

### Making Structs and Enums Pulic


We can also use `pub` to designate structs and enums as public, but there are a few extra details to the usage of `pub` with structs and enums. If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis. 

Filename: src/lib.rs

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
```

In contrast, if we make an enum public, all of its variants are then public. We only need the pub before the enum keyword.

Filename: src/lib.rs

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## 7.4 Bringing Paths into Scope with the `use` keyword

Having to write out the paths to call functions can feel inconvenient and repetitive. Whether we chose the absolute or relative path to the `add_to_waitlist` function, every time we wanted to call `add_to_waitlist` we had to specify `front_of_house` and `hosting` too. Fortunately, there’s a way to simplify this process: We can create a shortcut to a path with the `use` keyword once and then use the shorter name everywhere else in the scope.


Filename: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### Creating Idiomatic use Paths

Filename: src/lib.rs

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
```

Although both code snippets accomplish the same task, the more idiomatic way is bringing the function's parent module into scope with `use`. Specifying the parent module when calling the function makes it clear that the function isn't locally defined while still minimizing repetition of the full path.

On the other hand, when brining in structs, enums, and other items with `use`, it's idiomatic to specify the full path.

Filename: src/main.rs
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

The exception to this is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.

Filename: src/lib.rs
```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
```

If instead we specified `use std::fmt::Result` and `use std::io::Result`, we’d have two Result types in the same scope, and Rust wouldn’t know which one we meant when we used `Result`.


### providing new names with the `as` Keyword

Filename: src/lib.rs
```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Re-exporting Names with `pub use`

When we bring a name into scope with the `use` keyword, the name is private to the scope into which we imported it. To enable code outside that scope to refer to that name as if it had been defined in that scope, we can combine `pub` and `use`. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

Filename: src/lib.rs
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant metaphor, the people running the restaurant think about “front of house” and “back of house.” But customers visiting a restaurant probably won’t think about the parts of the restaurant in those terms. With pub use, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library.

### Using External Packages


In Chapter 2, we programmed a guessing game project that used an external package called rand to get random numbers. To use rand in our project, we added this line to *Cargo.toml*:

Filename: Cargo.toml
```toml
rand = "0.8.5"
```

Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the `rand` package and any dependencies from crates.io and make `rand` available to our project.

Then, to bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the crate, `rand`, and listed the items we wanted to bring into scope.

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps: listing them in your package’s *Cargo.toml* file and using `use` to bring items from their crates into scope.

Note that the standard `std` library is also a crate that’s external to our package. Because the standard library is shipped with the Rust language, we don’t need to change *Cargo.toml* to include `std`. But we do need to refer to it with `use` to bring items from there into our package’s scope. For example, with `HashMap` we would use this line:

```rust
use std::collections::HashMap;
```

### Using Nested Paths to Clean Up `use` Lists

If we’re using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space in our files.

Filename: src/main.rs
```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that different

Filename: src/main.rs
```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

We can use a nested path at any level in a path, which is useful when combining two use statements that share a subpath. 

Filename: src/lib.rs
```rust
use std::io;
use std::io::Write;
```

to

```rust
use std::io::{self, Write};
```

### Importing Items with the Glob Operator

If we want to bring *all* public items defined in a path into scope, we can specify that path followed by the `*` glob operator:

```rust
use std::collections::*;
```

This `use` statement brings all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined. Additionally, if the dependency changes its definitions, what you’ve imported changes as well, which may lead to compiler errors when you upgrade the dependency if the dependency adds a definition with the same name as a definition of yours in the same scope, for example.

## 7.5 Separating Modules into Different Files

So far, all the examples in this chapter defined multiple modules in one file. When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.

We’ll extract modules into files instead of having all the modules defined in the crate root file. In this case, the crate root file is *src/lib.rs*, but this procedure also works with binary crates whose crate root file is *src/main.rs*.


Filename: src/lib.rs
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

Filename: src/front_of_house.rs
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

Note that you only need to load a file using a `mod` declaration once in your module tree. Once the compiler knows the file is part of the project (and knows where in the module tree the code resides because of where you’ve put the `mod` statement), other files in your project should refer to the loaded file’s code using a path to where it was declared. In other words, `mod` is *not* an "include" operation that you may have seen in other programming languages.

Filename: src/front_of_house.rs
```rust
pub mod hosting;
```

Filename: src/front_of_house/hosting.rs
```rust
pub fn add_to_waitlist() {}
```

### Alternative File Paths
So far we’ve covered the most idiomatic file paths the Rust compiler uses, but Rust also supports an older style of file path. For a module named `front_of_house` declared in the crate root, the compiler will look for the module’s code in:

- *src/front_of_house.rs* (what we covered)
- *src/front_of_house/mod.rs* (older style, still supported path)

For a module named hosting that is a submodule of `front_of_house`, the compiler will look for the module’s code in:

- *src/front_of_house/hosting.rs* (what we covered)
- *src/front_of_house/hosting/mod.rs* (older style, still supported path)

If you use both styles for the same module, you’ll get a compiler error. Using a mix of both styles for different modules in the same project is allowed but might be confusing for people navigating your project.

The main downside to the style that uses files named *mod.rs* is that your project can end up with many files named *mod.rs*, which can get confusing when you have them open in your editor at the same time.
