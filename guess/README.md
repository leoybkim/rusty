# 1. Getting Started

## 1.1 Installation
- `$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`
- `rustc -- version`
- `rustup update`

## 1.2 Hello, World!
- rust files use extension `.rs`
- convention to use underscore in file names
- `rustc main.rs` to compile, then `./main` to run
- `cargo fmt` to format to standard style
- `println!` calls Rust macro; `println` without the `!` would call function instead

## 1.3 Hello, Cargo!
- build sytem and package manager
- `cargo new rusty`
- `Cargo.toml` is configuration files
    - `[package]` section requires name, version and edition for Cargo to compile the program
- Convert existing project to use Cargo with `cargo init`
- Build with `cargo build`
    - creates executable file in `target/debug/`
- Compile and run in one command using `cargo run`
- Can check if code can compile without buildling with `cargo check`
- Compile with optimization (at run time) with `cargo build --release`
    - will take longer to compile
    - creates executable in `target/release`

# 2. Programming a Guessing Game
- The `io` library comes from the standard library `std`
    - `use std::io`
- Rust has set of items defined in standard library that i brings into the scope of every program know as the *prelude*
- If a type you want to use isn't in the prelude, you have to bring that type into scope explicitly with a `use` statement
- `main` function is the entry point into the program
- `fn` declares a new function, `()` for empty parameters, `{}` the body of the function
- `let` statements create variable
    - variables are immutable by default
    - to make variable mutable, add `mut` before the variable name
- `//` starts comment
- without importing the `io` module with `use std::io`, you can still use the function with `std::io::stdin`
- `&` indicates that this argument is a reference, which let other code access that data without needing to copy the data into memory multiple times.
    - like variables, references are immutable by default unless `&mut`
- `read_line` puts the user input into the string passed to the function param, and it also returns a `Result`, which is an *enum*, which is a type that can be in multiple possible state called *variant*
    - `Result`'s variants are `Ok` and `Err`
    - if you don't call `expect`, the program will compile but with a warning
- `{}` can be used as a placeholder when printing values in `println!`
- the project is a *binary crate* which is an executable
- the `rand` crate is a *library crate* which contains code that is intended to be used in other programs and can't be executed on its own
    - modify *Cargo.toml* to include the `rand` crate as a dependency
    - Cargo understands Semantic Versioning and by default uses `^`
    - Cargo fetches the dependency from the *registry*, which is a copy of data from Crates.io where people in the Rust ecosystem post their open source Rust projects for others to use
    - Rust creates *Cargo.lock* file the first time you run `cargo build`, and when you build the project in the future, Cargo will see that *Cargo.lock* file exists and will use the versions specified, which lets you have a reproducible build 
    - run `cargo update` when you do want to update a crate
    - `cargo doc --open` will build documentation provided by all your dependencies locally
- `std::cmp::Ordering` is another enum that has variants `Less`, `Greater`, and `Equal`
- `match` expression is made up of *arms* which consists of *pattern* to match against
- A few of Rust's number types can have a value between 1 and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit number and more
    - by default Rust uses an `i32`
- *Shadowing* let us reuse the variable which is often used to convert a value from one type to another type
- the `:` after variable name tells Rust we'll annotate the variable's type

