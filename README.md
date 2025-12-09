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
