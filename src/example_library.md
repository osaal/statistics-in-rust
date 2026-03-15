# Building a Toy Statistical Library

In this chapter, we will put our newfound Rust abilities to the test and write our very own arithmetic mean function! This might sound simple, and the mathematics behind it are simple on purpose, but the actual implementation will use some fairly advanced Rust techniques.

Prerequisite knowledge for this chapter is a working understanding of Rust syntax, which you can get by reading the previous chapter. This chapter will teach you concepts such as unit tests, functional programming through iterators, as well as trait bounds and generic programming. No knowledge in these topics is required to follow the tutorial.

## How to Read the Tutorial

**Bolded words** highlight important words or concepts. They should be explained in the surrounding text.

Footnotes are small hyperlink numbers next to words, that take you to the end of the chapter[^0]. They offer additional information or small anecdotes, and can be skipped if wanted.

Any reference to code objects (functions, traits, types, etc.) is written in `monospace font`. The code we will be developing is presented in code blocks like the one below, with syntax highlighting.

```rust
fn hello_world() -> String {
    String::from("hello, world!")
}
```

Unless explicitly stated, all Rust code blocks are fully compilable and tested. This means that you can copy-paste the code into a code editor, and the Rust compiler will compile it. This promise holds *only* for Rust code.

The following highlight boxes are spread throughout the tutorial:

> [!NOTE]
> These are notes, offering extra information. Skipping these should not affect your understanding of the tutorial.

> [!TIP]
> These are tips, and can be helpful if you run into some common issues.

> [!WARNING]
> These are warnings, reminding you to be careful and diligent with the surrounding section.

> These boxes are reserved for notices that an above code block does not compile and that the block is only intended for educational purposes.

[^0]: Like this one.

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

For this statistical library, our MVP will be one function: `mean()`. It should take a suitable data type and return the arithmetic mean of it.

To keep things very simple, let's assume that the user always has a vector of `usize` integers, since the assumption is both somewhat reasonable and allows for a very simple backbone. We can call the input simply `x`:

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
> **What's with the `0.`?**
> 
> The Rust compiler makes sure that we actually return what we declare the return type to be. Without returning something that can be interpreted as an `f64`, the compiler will error - hence the placeholder value `0.`, which the compiler interprets as a float. We will replace it with a real value once we get further.

However, what if the input vector is empty? To be safe, let's wrap the return in a `Result` type and create a matching error type for the function to use:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v3}}
```

Wait, that's a **lot** of extra code - what just happened? Let's work through it line by line:

-   The first two lines are `use` statements, telling the compiler that we want to use items from other crates. In this case, we are using two standard-library items: the [`Error` marker trait](https://doc.rust-lang.org/std/error/trait.Error.html) and the [`Display` trait](https://doc.rust-lang.org/std/fmt/trait.Display.html) that `Error` depends upon.
-   After that, we have our new `mean` function. We have to wrap the placeholder return with `Ok()` since we return a `Result<f64, MeanError>`, not just a `f64`.
-   After that, we have the new error type `MeanError` that derives the [`Debug` trait](https://doc.rust-lang.org/std/fmt/trait.Debug.html).
-   Following the enum, we implement the `Error` marker trait in one line. This tells the Rust compiler that our `MeanError` is to be treated like an error type in general.
-   Finally, we have a `Display` implementation for our error type `MeanError`. This implementation requires us to implement a function called `fmt()` - for the time being, we simply added a [`todo!()` macro](https://doc.rust-lang.org/std/macro.todo.html) inside it. This satisfies the compiler, but if the function is ever called, the program will crash - okay for the time being, but not the finished product.

> [!TIP]
> **How am I supposed to remember all of this?!**
>
> That's the neat part, you don't. Let the compiler tell you what to do instead: start by adding the `MeanError` definition. Then try to add the `impl Error` line. If you try to compile the code, the compiler will error and tell you that "`MeanError` doesn't implement `std::fmt::Display`, which is "required by a bound in `std::error::Error`".
> 
> This means that you should implement Display yourself - try to add it as just a single line `impl Display for MeanError {}`. Now the compiler will tell you that "not all trait items implemented, missing: `fmt`". It even gives you the code snippet needed to satisfy the compiler, complete with the `todo!()` placeholder!
> 
> Finally, if you follow the compiler, you still get an error on the Error impl (pun unintended), that "`MeanError` doesn't implement `Debug`", required by the `Error` trait. Once again, follow the compiler's instructions - et voilá, you're done.

> [!TIP]
> **VSCode and `rust-analyzer`**
> 
> To make things even easier: if you are using VSCode as your editor, you can install the third-party plugin [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer). Despite being a plugin, it is actually developed by the team behind the Rust language.
> 
> With this plugin, you get a few extremely nifty features: the plugin will automatically try to compile your code whenever you save a file, showing you errors and warnings as squiggly lines in the code. Hovering over them lets you see the original compiler error message.
> 
> Pressing `<Ctrl + .>` while your cursor is adjacent to the problem area will give you Quick Fixes, which can automatically implement the fix that the compiler is suggesting.
> 
> Give it a try - it will speed up development dramatically!

Going forward, I will be hiding implementations that we have completed, and only showing the code that we are working on. Of course, if the order or placement of the code matters, I will show the relevant context so that you know where to edit the code. If you lose track, you can always check the source code at the very end of the chapter, which shows you the final results.

Okay, now that we have the signature of our `mean()` function, let's make it actually calculate a mean. As you (hopefully) know, an arithmetic mean is simply the sum of the elements divided by their counts. Since we defined `x` to be `Vec<usize>`, we can rely on a few methods afforded to us by `Vec`. If you wish, you can take a look at the [documentation for `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) and see if you can find the relevant methods before going any further.

The first one we might be interested in is `len(&self) -> usize`. The documentation states that it gives the number of elements in the vector, also known as the vector's length:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v4}}
```

Now, how do we sum all the elements together? If you've worked with statistical data analysis before, you've probably used either of two patterns: **for-loops** or **folds**. The latter is more idiomatic to Rust, but also more complex, so let's start with a for-loop.

Vectors can be indexed into by the element index, meaning that we can retrieve the *N*th element at a time. A for-loop is defined using the syntax `for VAR_NAME in RANGE`, where the range by default is right-hand exclusive and written as `start..end`[^1]. Thus, we can access each element with:

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
> The answer is **integer overflow**. Since `x` can take any amount of `usize` elements, they may sum up to be more than the maximum size of `usize`, causing the program to crash. This is why this is a **toy** library, not a real one - a reliable implementation of a mean function should use safeguards to make sure that the intermediate result cannot overflow, so that we could reliably calculate means for very large numbers without worrying about our program crashing. For this tutorial, this will have to suffice.

> [!NOTE]
> We only defined `tally` to be `0`, but never gave it a type - that could be any of the unsigned or signed integers! However, the compiler is smart enough to infer from the later code in the loop that we will be adding elements of type `usize` to it, so it makes the type `usize` to match. Neat, huh?

Finally, we can divide `tally` by `length`, and return the result wrapped in an `Ok()` variant:

```rust
{{#include ../examples/ex_library_1/src/lib.rs:v7}}
```

Note, that we force-converted the elements into `f64` before the division. By default, [the division creates the same type](https://doc.rust-lang.org/std/ops/trait.Div.html#impl-Div-for-usize), a `usize`. However, that will truncate a potential decimal point to fit it inside a `usize` (which has no notion of decimals). An `as` conversion has higher **precedence** than the division operator `/`, which is why we can convert on the same line without using parentheses.

> [!WARNING]
> Once again, there are dangers afoot. This time, try to come up with what could go wrong with force-converting a `usize` to an `f64`.

There we go, a functioning `mean()` function... or wait, does it actually function? How do we know that it does what it should do?

[^1]: When writing this section, I actually made an elementary code error called an **off-by-one** error. In the original code, I wrote `for i in 0..length - 1`, since I remembered that indices start at zero, and thus end at `length - 1`. This is true, *but* in Rust, ranges are right-exclusive, so the end of the range becomes `end - 1`, or in this case, `length - 1 - 1`. Ranges can be defined as right-inclusive with the syntax `start..=end`, but this is less often used. This is a reminder to always test your code, and be humble - even an experienced data programmer *will* make mistakes.

## Testing Our Functions

When writing compiled code, and especially when writing library code (as opposed to an executable binary), actually testing the code can become a bit complicated. Thankfully, Rust provides us with built-in tools for writing **unit tests**. Unit tests are small test functions that make sure that a given piece of code does what it is supposed to do. They are called "unit" tests, because they are supposed to be ran on self-contained units of code, i.e., semantically meaningful fragments of code.

Before we start, a comment on testing: code testing is less important when you are writing analysis code for a research paper, since in a sense, the final output of your analysis *is* the test itself. However, if or when you start writing your own analysis functions (e.g., an implementation of a particular statistical method), it is *imperative* that you make sure that the code is doing exactly what it is supposed to do. Because of this, it is good to get into the habit of writing tests.

> [!TIP]
> **Test-driven development**
> 
> There is a philosophy of computer science that roughly states that untested code is unfinished code. In fact, there is a whole decicated way of writing code called *test-driven development*, where (theoretically) every piece of code gets tested as you write it.
> 
> To simplify the procedure: you start by writing a test that tests for the behaviour you want, without actually writing the code to complete the behaviour. You then run the test, which naturally fails immediately. Then, you write the *minimum* amount of code that fulfills the test requirement, making the test pass.
> 
> This process is repeated, refining the test each time to be more complex, or by adding new and obscure test cases. Finish once you have enough confidence that you have covered most reasonable scenarios - your code is now finished. Try the procedure out if you wish, or search for test-driven development to learn more.

To write unit tests in Rust, we define a new module in the project crate called `tests` and tag it with a special annotation:

```rust
{{#include ../examples/ex_library_2/src/lib.rs:v1}}
```

The annotation `#[cfg(test)]` tells the Rust compiler to include this source code only when building against a **test target**. In practice, this means that the code will not be included in any potential binaries that a user of our library might create, but will be included whenever we run `cargo test` on the crate.

To have access to our defined function from before, we call `use super::*;`. Remember, that modules are tightly scoped, so that anything in the parent scope is unavailable to the child module unless explicitly brought in - `super::*` brings in everything in the parent crate.

We can now add our first test function, making sure that the `mean()` function gives a sensible result on a simple test case:

```rust
{{#include ../examples/ex_library_2/src/lib.rs:v2}}
```

A test is a regular function, except with a few special properties:

1.  We always tag the test with the `#[test]` attribute to tell the compiler that it is a test.
2.  The test function returns nothing (or technically, the unit type).
3.  We use one or more **assertions** in the test to, well, test the values that we want.

Assertions are macros that are guaranteed to be ran at all times, and that, if failing, will cause the program to crash. In this case, we *want* it to crash - Cargo knows that a crashing test is a failed test, and let's us know.

Assertions come in many varieties, but the most common ones are [`assert!()`](https://doc.rust-lang.org/std/macro.assert.html#) and [`assert_eq!()`](https://doc.rust-lang.org/std/macro.assert_eq.html). The former checks that whatever expression is ran inside it evaluates to `true`, or crashes. The latter checks that the first and second expressions evaluate to the same type and value, or crashes.

> [!NOTE]
> In order to use `assert_eq!()`, the type that comes out of both expressions must implement [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) against itself, i.e., `A == A`.

In this case, we are creating a small vector of the values 1, 2, and 3. We then call our mean function on the vector. Finally, we assert that the unwrapped value of our output is equal to the floating-point value `2.0` (remember, the compiler is smart and assumes that `2.` is of type `f64`, since `output.unwrap()` returns `f64` in this case).

And that's it! Now, we can run `cargo test`, and observe the following output:

```
(unnecessary details removed from the output)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/lib.rs (/very/long/file/path)

running 1 test
test tests::test_mean ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

That's a lot of stuff, but the most important part is in the middle: `test result: ok. 1 passed`. We can also see which tests passed and which failed. If the test would have failed, we would see the two comparison values of `assert_eq!()`, which helps in debugging whether the testing code or the actual code is wrong.

Let's try our hand at test-driven development, and write a test that we want to account for. Currently, our code does not actually handle the case where `x.len() == 0`. Let's write a function that tests for that:

```rust
{{#include ../examples/ex_library_2/src/lib.rs:v3-1}}
```

> The above example will not compile on its own, without further modifications to the code.

We now have a simple test: try to calculate a mean on an empty vector. Since the vector did not contain any data for the compiler to use in type inference, we had to manually specify that its data type is `Vec<usize>`.

Instead of a single assertion, we now do two assertions: firstly, is the result an error, and secondly, is the error the specific enum variant `MeanError::DivideByZero`? You can, in fact, chain as many assertions as you want - the first one to fail will be the one that is reported.

In order to make the code compile, we need to implement the changes to `MeanError` that allow for these assertions:

```rust
{{#include ../examples/ex_library_2/src/lib.rs:v3-2}}
```

There are three major changes:

1.  We derived `PartialEq`, which is necessary for `assert_eq!()` to work. Since we compare `MeanError` with itself, we do not need a custom implementation.
2.  We added a new enum variant, `DivideByZero`. You can call it whatever you like, but a good error enum variant describes what happened.
3.  We replaced the `todo!()` part of the `Display::fmt()` implementation with a match statement, and added one new arm.

The match statement might require a bit more explanation. `Display::fmt()` wants to return a [`std::fmt::Result`](https://doc.rust-lang.org/std/fmt/type.Result.html), which is a specific type meant for things written to the terminal screen. To make one, we can use the [`write!()` macro](https://doc.rust-lang.org/std/macro.write.html), which takes a formatter object (given to us by the function parameter `f`) and a string to be formatted. This formatting string can be very complex, but at its simplest, it is just a `str`.

We match against the different possible `MeanError` variants, which in this case is just one. Upon matching it, we call `write!()` with the function parameter `f`, as well as a string that informs us *why* the error happened.

> [!TIP]
> **Rule of thumb for error design**
> 
> The error enum variant should state in plain language *what* happened, the `Display` message should state *why* it happened.

Go ahead and run `cargo test`, and... it fails:

```
(truncated output)
running 2 tests
test tests::test_mean ... ok
test tests::divide_by_zero ... FAILED

failures:

thread 'tests::divide_by_zero' (14335) panicked at my_library/src/lib.rs:127:13:
assertion failed: output.is_err()
```

Of course it fails - we haven't changed the `mean()` function to actually account for zero-length vectors! The code is trying to divide by zero, which in Rust is an instant crash.

To make the test work, we need to catch whether `len() == 0`, and if it is, return an error instead of a correct value:

```rust
{{#include ../examples/ex_library_2/src/lib.rs:v4}}
```

We simply add an `if` statement checking for the length of the vector, and if it is zero, we return our error type variant (wrapped in `Err()`). If not, this block will not execute and the code will work as before.

Finally, let's run `cargo test` again...

```
(truncated output)
running 2 tests
test tests::divide_by_zero ... ok
test tests::test_mean ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Success, the test passes! This means that our code correctly recognizes an empty vector and returns the error type that we want it to return, instead of crashing out completely.

> [!TIP]
> **Why is the order of tests in the output different?**
> 
> Cargo tries to run tests in parallel or concurrently as much as possible. This means that tests will execute at the same time (or close enough). Depending on a *lot* of things, including the complexity of the tests, they may finish at different times. Thus, the order of the tests in the output can differ from test run to test run.

Let's take inventory of what we have now, and compare it to our MVP from the start of the chapter. We have a `mean()` function that takes a vector of unsigned integers of system length. If that vector is empty, the function returns an error type. Otherwise, it accumulates the elements using a for-loop, divides the result with the length of the vector with type conversion, and returns the final result.

Wait, we have actually fulfilled our MVP - and even gone beyond it. Congratulations!

Next up, let's see if we can't make this function prettier, more idiomatic, and more usable.

## Functional Code

As a reminder, here is our current `mean()` function:

```rust
{{#include ../examples/ex_library_3/src/lib.rs:v1}}
```

I mentioned previously that there are two main ways to iterate through elements in a container (e.g., a vector), a for-loop and a fold. We implemented the prior, but the latter is much more idiomatic to Rust.

**Folds** (also called reductions) are a common pattern in functional programming. The idea is that we apply a function to each element of the collection, returning the collection less the "used" element as well as an **accumulator**. If you think about it, our for-loop does something similar: for each element in the vector, it adds the element to the previous tally.

The difference is slight, but it can have surprising consequences: for a for-loop to work like a fold, we need a mutable variable outside the scope of the for-loop. This makes the variable potentially mutable even *after* having done the for-loop. For instance, a line of code after the loop could reset the tally to zero, regardless of what it contains.

A fold does not allow this: the accumulator is internal to the fold only, and what is returned is functionally (pardon the pun) immutable. The end result can be stored immutably, so that nothing can affect it. *Voilà*, guaranteed function purity!

In Rust, vectors do not implement a folding method automatically. However, vectors implement a trait called [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html), which allows them to become something implementing the [`Iterator` trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html) - which, in turn, *does* allow for folding! This conversion is done by calling `Vec.into_iter()`.

The `Iterator` trait has 76 (!) different methods, but right now, we are only interested in one: [`Iterator.fold()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold). Hold on to your hat, because a lot of new stuff is about to come up. Let's start by looking at the function signature for `fold()`:

```rust,ignore
fn fold<B, F>(self, init: B, f: F) -> B
where
    Self: Sized,
    F: FnMut(B, Self::Item) -> B,
```

That... is a mouthful. The method is generic, meaning that all of its inputs and outputs can take multiple different type forms, as long as they satisfy certain **trait bounds**. In this case, the method takes two inputs and returns one output[^2]: an `init` object, whose type must be the same as the output, and an `f` object.

The `f` object is what is called a **closure**. In some languages, this is also called an **anonymous function**. A closure is a function written in special syntax:

```rust,ignore
let f = |a, b| a + b
```

The idea of closures is that **functions are data types themselves**. This might sound confusing, but it is actually extremely powerful, as it allows us to have functions that create other functions (so-called **functors**), as well as pass functions around as data between different functions, or even store functions in vectors.

In the case of `fold()`, `f` is a closure that implements `FnMut(B, Self::Item) -> B`. Breaking this line apart, `f` must be a [function that takes its input mutably](https://doc.rust-lang.org/std/ops/trait.FnMut.html), i.e., is allowed to mutate the input. The input must have some associated type `Self::Item` (which we can determine ourselves), as well as a starting item `b`. Finally, it must return something of type `B`, i.e., the same type that `init` was set to, and the same type that the closure took as its first parameter.

What does all of this do? Well, `fold()` starts with the value of `init`, and for each element in the parent `Iterator` object, it applies the closure `f`. The return of that closure is used as the `init` value for the next iteration, i.e., for the next element in the iterator. Once there are no more iterators, the final output value of the closure `f` is returned.

But how do we know what the closure is supposed to look like? We can take a peek at the documentation. Quote:

> `fold()` takes two arguments: an initial value, and *a closure with two
arguments: an ‘accumulator’, and an element*. The closure returns the value that
the accumulator should have for the next iteration.
> 
> (Cursive added. Source: [std::iter::Iterator::fold()](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold))

Thus, we can directly replace our for-loop with the folding pattern[^clippy-1]:

```rust
{{#include ../examples/ex_library_3/src/lib.rs:v2}}
```

To make sure we understand what is going on, let's work through the new one-liner:

1.  We take `x` and we call it's `into_iter()` method. We know this exists since `x` is restricted to be a `Vec`.
2.  The method returns some type of iterator, which we don't know[^fn]. The only thing we need to know is that whatever type it is, it implements `Iterator`. Therefore, we can chain a call to `Iterator::fold()`.
3.  We tell the fold to start with `init = 0`, which the compiler can infer to be the same type as the return type of the closure as well as the first element of the closure parameter list, i.e., the accumulator.
4.  We then give fold a closure with two parameters inside vertical bars: an accumulator and an element.
5.  Finally, we define the closure to sum the accumulator and the element together and return the result in one line.

What will happen? Let's think through the case of `x == vec![1, 2, 3]`:

1.  The vector is turned into an iterator, which yields one element at a time.
2.  Fold starts with the closure arguments `|0, 1|`. The first argument is `init`, the second is the first element of the iterator, `1`.
3.  Fold sums them together and returns the result `0 + 1 = 1`.
4.  Fold then takes the result `1` and starts the second closure iteration with the arguments `|1, 2|` - the previous result, and the second iterator element.
5.  Fold calculates `1 + 2 = 3`and returns the output.
6.  Fold runs a third time, with `|3, 3|`, and returns `6`.
7.  Finally, fold sees that there are no more elements, so it returns the final result, `6`.

> [!NOTE]
> We could, in theory, even include the length calculation in the closure. One way to do this is to return a tuple, where the first number represents the number of times the closure has been called (incremented inside the closure), and the second represents the actual closure.
> 
> However, why would this be problematic in our case? Think about what information we need in order to safely execute the function.

Iterators are, in general, much more efficient than for-loops. In many cases, method chaining could imply that we need to iterate through the dataset multiple times. For instance, we might first want to select a subset of the data, then a subset of that subset, then do some transformation, then finally calculate a single value out of the data. However, Rust iterators are (for the most part), **lazy**: they will not actually do anything until a **collection method** is called on them. The two most common collection methods are [`.next()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next) and [`.collect()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect). The former yields the next element in the iterator, and allows you to "step through" the iterator one element at a time. The latter runs through the entire iterator, applies everything that was added in the chain, and returns the final output in a collection type (usually a vector).

Before we're done here, I want to mention some other traditional functional patterns, all implemented for `Iterator`:

-   [`zip()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip) combines two iterators A and B into an iterator whose elements are a tuple (A, B). In other words, the number of elements stays the same, but the elements are paired up. Zips can be undone with [`unzip()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.unzip), yielding two iterators.
-   [`map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) calls a given closure on each element of the iterator, returning the result. Note, that simply calling `iter.map()` will not actually yield anything. You have to either iterate through the elements (with `.next()` or `.collect()`), or do something else with the returned iterator before iterating through everything. This allows for chaining multiple operations before actually executing all of them.
-   [`for_each()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.for_each) is similar to `map()`, except that it does not return anything. It is used for **side effects**, i.e., when the closure does something outside of the function (such as sends a message to a server).
-   [`filter()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) applies a predicate closure to each element, returning `true` or `false` for each element. Like `map()`, the result needs to either be collected somehow, or transformed into another iterator with another functional pattern. As the name suggests, this can be used to conditionally select elements for further analysis, such as "select only data above the mean".
-   [`filter_map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map) combines `filter()` and `map()`. Its closure has to return an Optional value, and once collected, only the `Some()` values are actually modified by the mapping part of the closure. It can be used to replace a chain of `iter.map().filter().map()` in one go.
-   [`enumerate()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) does what the above infobox suggested we could, but won't, do: it creates an iterator that contains a tuple of `(i, item)`, where `i` is the index of the iteration.
-   [`flatten()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flatten) takes an iterator and removes all nested iterator structures. For instance, if you previously mapped a closure that returned iterators for each element, `flatten()` will flatten those returned iterators back into a single big iterator.
-   [`flat_map()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map) functions like `iter.map().flatten()`.

There are many more methods, and I encourage readers to get familiar with them. Perhaps one day this book might contain a full listing and description of them, but not today...

Next up, we are going to get generic.

[^2]: Technically, `self` is also an input with the type bound of being `Sized`, but we will ignore this for now - the `Sized` trait is fairly advanced Rust knowledge.

[^fn]: The type of iterator that `Vec.into_iter()` returns is contingent on how `Vec` implements `IntoIter`. The point here is that whatever it is, it too implements `Iterator`, so we can safely use its iterator methods without knowing its specific type. If you wish, you can look at the [source code](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html#3806) to figure it out, but I must warn you - it's deep Rust wizardry.

[^clippy-1]: This fold could actually be expressed even simpler, using [`Iterator.sum()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) with the turbo-fish operator: `x.into_iter().sum::<usize()`. In general, if there exists an implementation in the standard library, you should use it (they're wizards over there, they know what they're doing). For pedagogical reasons, however, we will stick to our implementation.


## Generic Functions

Once again, let's remind ourselves of our function:

```rust
{{#include ../examples/ex_library_4/src/lib.rs:v1-1}}
```

Wouldn't it be nice if we could allow other collection types than just `Vec<usize>`? With trait bounds, we can do just that!

In the previous section, we learned how trait bounds are used in the standard library to make trait methods generic. Turns out, a *lot* of things can be made generic, including function inputs and outputs.

Before we start: what does **generic** mean? Rust uses a process called **monomorphisation**. Whenever the compiler finds a generic trait bound, it looks for every place in the codebase where the generic item is actually used. It checks what types are used with it, as well as that they fulfil the trait bound requirement. If everything is okay, it creates a new version of the item, with the types specified to the types that were using it. For instance, the following function:

```rust
{{#include ../examples/ex_library_4/src/lib.rs:v1-2}}
```

when used with the types `i32` and `String` would turn into something like:

```rust,ignore
    fn do_something_1(x: i32) { ... }
    fn do_something_2(x: String) { ... }
```

> The above example will not compile. It is a simplification of compiler-generated code during monomorphisation.

This is nice, because it does two things: firstly, we do not need to write unique functions for every use case ourselves, and secondly, we can implement the function for *any* `T` that implements `Debug`, even when we have no idea what type `T` actually is!

In order to make our function generic over different types of `x`, we therefore need to identify which traits of `x` are necessary for our function to work. We can do that by looking at how we use `x`:

-   Firstly, we call `x.len()`, meaning that `x` must have a `.len()` method.
-   Secondly, we call `x.into_iter()`, meaning that `x` must have an `.into_iter()` method.
-   Finally, our fold must have the same type in the `init` as the accumulator and the element, i.e., we need to bind the item type contained in `x` to `usize`.

Those are the only requirements our function has for `x`! Let's deal with the second one first, since it actually also deals with the third, and is the simpler one.

The method `into_iter()` is offered by the trait `IntoIterator`. Thus, if `x` is guaranteed to be `IntoIterator`, the method will be available regardless of the type of `x`. Further, looking at the [documentation for `IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html), we see that it has two associated types, the former of which is important to us:

```rust,ignore
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    // Required method
    fn into_iter(self) -> Self::IntoIter;
}
```

An **associated type** is a generic type connected to a trait. This generic type is used to set further bounds inside the trait. In our case, the type `Item` is used to make sure that whatever `Iterator` object the method `into_iter()` returns, its items must be of type `Item`. Thus, we can use this type to restrict `x`'s item types: `x: T where T: IntoIterator<Item = usize>`.

However, if we tried to just add that, our library would not compile. The reason is that we have not guaranteed that `x` has a `len()` function.

Here, we hit a little snag. Looking at the documentation, [`len()`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html#method.len) is actually a method for the `Vec` type, not a trait method[^fn-2]. Thus, we cannot give our function a trait bound that guarantees the availability of `len()`, except for requiring `Vec` types. However, we can work around this, but we need to make an important decision.

An iterator cannot know its own length off the get-go, because at any point in time, an iterator only contains the next item. Thus, to count the "length" of an iterator, we would need to iterate through all of the elements using the `Iterator::count()` method. This will (probably) increase our processing time as compared to using a vector. However, it gives us the genericity we desire, so that users could use any item, as long as it converts into an iterator.

To not belabour the point: we will decide to make this trade-off for the sake of pedagogy. However, in the real world, we would have to consider the use cases more thoroughly, especially if we were concerned about optimal speed. Generally speaking, making things generic through trait bounds will usually result in larger binaries when compiled as well as potential slowdowns, but the added flexibility can still be worth it.

Let's add the trait bounds and change the code to work with iterators:

```rust
{{#include ../examples/ex_library_4/src/lib.rs:v2}}
```

You might notice I snuck in another type restriction - I'm sorry, I couldn't help myself. The reason is that [`Iterator.count()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count) consumes the iterator, and we need to walk through it twice - once for the length, and once for the tally. Therefore, by adding the extra restriction that `x` must be cloneable, we can call `.clone()` on it to get a copy of the data. 

Let's make sure that our implementation works by adding a test and running it. We can use another collection data type that already implements `IntoIterator`, [the simple array](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html#impl-IntoIterator-for-%26%5BT%5D):

```rust
{{#include ../examples/ex_library_4/src/lib.rs:v3}}
```

Here, we make sure that the output is exactly the same, regardless of if we use a vector or a slice. Lo and behold, it is:

```
(truncated output)
running 3 tests
test tests::divide_by_zero ... ok
test tests::generic_input ... ok
test tests::test_mean ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

[^fn-2]: While writing this, I could not find a cleaner option for implementing a trait bound that offers `len()`. If anyone knows of, or finds, one - please let me know in the GitHub repository! I will gladly add it to the book.

## Finishing Up

We have accomplished quite a lot. Our statistical mean function is:

-   Generic over any data type that implements `IntoIterator<Item = usize> + Clone`
-   Checks for zero division errors and returns an appropriate error message if needed.
-   Calculates the arithmetic mean and returns a floating-point decimal of the result.

Naturally, there are many more things we could do. However, because nobody wants to read a 30.000-word tutorial essay, I will leave you with a few practice tasks. Do them if you wish, or start delving into other statistical work in R.

For readability, below is the entirety of our final code:

```rust
{{#include ../examples/ex_library_4/src/lib.rs:v4}}
```

### Further Practice 

The tasks are ordered approximately by difficulty.

1.  Right before returning the result, our `mean()` function forcibly converts the tally and lengths from `usize` into `f64`. This could pose an issue if [`usize > f64::MAX`](https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.MAX). If this happens, the program will crash. **How would you catch this issue and return an appropriate error instead?**
2.  For optimal flexibility, our `mean()` function should also work when used in functors such as `map()`, over an iterator of iterators (e.g., calculating the arithmetic mean for every variable in a dataset). **Try to use the function in such a context and see if it works as expected. Why or why not? If not, what changes would need to be made to accommodate this?**
3.  When we first implemented the tally using a for-loop, we noted how it runs the risk of overflowing `usize`. However, we no longer have the same implementation. **What would happen if the sum of all elements in `x` is larger than `usize::MAX`?** Try to use the documentation to solve the issue before testing it in code!
4.  There are multiple different means, not just the arithmetic mean. **Try to implement the [geometric mean](https://en.wikipedia.org/wiki/Geometric_mean) yourself, in the same way as the arithmetic mean.**
5.  (Hard) Let's take it a step further: **change it so that the `mean()` function can be used to calculate either the arithmetic or the geometric mean, depending on user selection.** Hint: Use an enum in the parameters as a "settings" object. Bonus points if you also implement a sensible `Default`.

If you want, you can leave your solution(s) to the problems in the [Discussions section](https://github.com/osaal/statistics-in-rust/discussions) of this book's GitHub repository. With your explicit permission (don't worry, I will ask before!), I would like to include some of them into a later version of the book as suggested answers.



