# Building a Toy Statistical Library

In this chapter, we will put our newfound Rust abilities to the test and write a toy statistical library. The library will include a few data transformation tools as well as statistical functions, and will showcase both basic and advanced Rust design patterns.

## Setup

If you want to follow along, create a new Cargo project using `cargo new --lib` (new directory) or `cargo init --lib` (existing directory). In the text, we will be referring to source files using file paths relative to the project directory (the directory where `Cargo.toml` exists). For instance:

-   If your project lives in `C:\Users\MyUser\Documents\MyProject\`, then a hypothetical path `./src/lib.rs` refers to `C:\Users\MyUser\Documents\MyProject\src\ļib.rs`.
-   If your project lives in `~/MyUser/MyProject`, then a hypothetical path `./src/lib.rs` refers to `~/MyUser/MyProject/src/lib.rs`.

> [!TIP]
> If you work on Windows, your file paths are always delimited by backslashes `\`. In Linux and MacOS, file paths are delimited by slashes `/`. In this book, we will be using slashes - remember to substitute file paths with the appropriate paths for your system.

Make sure to make a **library crate**, not a binary crate - we are creating a code library for reuse, not a binary executable.

When you're done, your project directory will look something like this:

```
my_project/
├── src/
│   └── lib.rs
└── Cargo.toml
```

Your Cargo.toml will look something like this:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2024"

[dependencies]
```

The file `./src/lib.rs` will contain a template for a function and a test suite. You can go ahead and erase everything in it - we will start from an empty `lib.rs` file.
