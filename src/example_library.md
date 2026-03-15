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

## Starting Out

When working with code, it's always best to start with a **minimum viable product (MVP)**, regardless of whether it fulfills all your hopes and dreams.

For this statistical library, our MVP will be three functions: `mean()`, `median()`, and `mode()`. Each of them should take a suitable data type and return one suitable value.

Let's start with the mean function. To keep things very simple, let's assume that the user always has a vector of `usize` integers, since the assumption is both somewhat reasonable and allows for a very simple backbone. We can call the input simply `x`:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v1}}
```

> [!NOTE]
> You will probably get a bunch of compiler warnings at this point related to unused functions and parameters. You can ignore these for now, as we will very soon deal with most of them.

What should we return? Well, since a mean entails division, there is the possibility that the result is a decimal number, so let's return an `f64`, the largest possible float:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v2}}
```

> [!NOTE]
> "What's with the `0.`?" The Rust compiler makes sure that we actually return what we declare the return type to be. Without returning something that can be interpreted as an `f64`, the compiler will error - hence the placeholder `0.`, which the compiler interprets as a float. We will replace it with a real value once we get further.

However, what if the input vector is empty? To be safe, let's wrap the return in a Result type and create a matching error type for the function to use:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v3}}
```

Wait, that's a **lot** of extra code - what just happened? Let's work through it line by line:

-   The first two lines are `use` statements, telling the compiler that we want to use items from other crates. In this case, we are using two standard-library items: the `Error` marker trait and the `Display` trait that `Error` depends upon.
-   After that, we have our new `mean` function. We have to wrap the placeholder return with `Ok()` since we return a `Result<f64, MeanError>`, not just a `f64`.
-   After that, we have the new error type `MeanError` that derives `Debug`.
-   Following the enum, we implement the `Error` marker trait in one line. This tells the Rust compiler that our `MeanError` is to be treated like an error type in general.
-   Finally, we have a `Display` implementation for our error type `MeanError`. This implementation requires us to implement a function called `fmt()` - for the time being, we simply added a `todo!()` macro inside it. This satisfies the compiler, but if the function is ever called, the program will crash - okay for the time being, but not the finished product.

> [!TIP]
> *"How am I supposed to remember all of this?!"*
>
> That's the neat part, you don't. Let the compiler tell you what to do instead: start by adding the `MeanError` definition. Then try to add the `impl Error` line. If you try to compile the code, the compiler will error and tell you that "`MeanError` doesn't implement `std::fmt::Display`, which is "required by a bound in `std::error::Error`".
> 
> This means that you should implement Display yourself - try to add it as just a single line `impl Display for MeanError {}`. Now the compiler will tell you that "not all trait items implemented, missing: `fmt`". It even gives you the code snippet needed to satisfy the compiler, complete with the `todo!()` placeholder!
> 
> Finally, if you follow the compiler, you still get an error on the Error impl (pun unintended), that "`MeanError` doesn't implement `Debug`", required by the `Error` trait. Once again, follow the compiler's instructions - et voilá, you're done.

> [!TIP]
> To make things even easier: if you are using VSCode as your editor, you can install the third-party plugin `rust-analyzer`. Despite being a plugin, it is actually developed by the team behind the Rust language.
> 
> With this plugin, you get a few extremely nifty features: the plugin will automatically try to compile your code whenever you save a file, showing you errors and warnings as squiggly lines in the code. Hovering over them lets you see the original compiler error message.
> 
> Pressing `Ctrl + .` while your cursor is adjacent to the problem area will give you Quick Fixes, which can automatically implement the fix that the compiler is suggesting.
> 
> Give it a try - it will speed things up dramatically!

Going forward, I will be hiding implementations that we have already done, and only showing the code that we are working on. Of course, if the order or placement of the code matters, I will show the relevant context so that you know where to edit the code. If you lose track, you can always check the source code of this book (link incoming), which will show you the final, ready-to-compile result.

Okay, now that we have the signature of our `mean()` function, let's make it actually calculate a mean. As you (hopefully) remember, a mean is simply the sum of the elements divided by their counts. Since we defined `x` to be `Vec<usize>`, we can rely on a few methods afforded to us by `Vec`. If you wish, you can take a look at the [documentation for `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) and see if you can find the relevant methods before going any further.

The first one we might be interested in is `len(&self) -> usize`. The documentation states that it gives the number of elements in the vector, also known as the vector's length:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v4}}
```

Now, how do we sum all the elements together? If you've worked with statistical data analysis before, you've probably used either of two patterns: **for-loops** or **folds**. The latter is more idiomatic to Rust, but also slightly more complex, so let's start with a for-loop.

Vectors can be indexed into by the element index, meaning that we can retrieve the Nth element at a time. Since we know the length of the vector, we know the maximum index (one less than the vector - remember that programming counting starts at zero, not one). Thus, we can iterate from zero to `len()-1` and do calculations on the elements:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v5}}
```

In order to sum the elements up, we can declare a mutable variable outside of the loop, set it to zero, and then add each element to the variable:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v6}}
```

> [!WARNING]
> This code will compile, but can be dangerous - can you see why?
> 
> The answer is **integer overflow**. Since `x` can take any amount of `usize` elements, they may sum up to be more than the maximum size of `usize`, causing the program to crash. This is why this is a **toy** library, not a real one - a reliable implementation of a mean function uses memory trickery to make sure that the intermediate result cannot overflow, so that we can reliably calculate means for very large numbers without worrying about our program crashing.

> [!NOTE]
> We only defined `tally` to be `0`, but never gave it a type - that could be any of the unsigned or signed integers! However, the compiler is smart enough to infer from the later code in the loop that we will be adding elements of type `usize` to it, so it makes the type `usize` to match. Neat, huh?

Finally, we can divide `tally` by `length`, and return the result wrapped in an `Ok()` variant:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v7}}
```

Note, that we force-converted the elements into `f64` before the division. By default, the division creates the same type, a `usize`. However, as we know, that will truncate a potential decimal point to fit it inside a `usize` (which has no notion of decimals). An `as` conversion has higher **precedence** than the division operator `/`, which is why we can convert on the same line without using parentheses.

> [!WARNING]
> Once again, there are dangers afoot. This time, try to come up with what could go wrong with force-converting a `usize` to an `f64`.

There we go, a functioning `mean()` function... or wait, does it actually function? How do we know that it does what it should do?

## Testing Our Functions


