# From R to Rust

This chapter will explain some of the most important changes when you move from R to Rust for statistical programming.

## Rust is Compiled, Not Interpreted

One of the most fundamental differences to R is that Rust is a *compiled* language, while R is an *interpreted* language.

In R, you write script files which are then invoked by an interpreter. The R interpreter reads the source code and executes it on-the-fly. This allows for fast code running, but makes things like hard typing (see below) much more difficult.

If you want to give your code to another person, they have to have the `.R` script files as well as the installed R interpreter in order to run it.

In contrast, Rust is compiled. This means that the source code `.rs` files are given to a compiler, which translates them into machine code and produces an *executable binary*, i.e., an `.exe` file on Windows, an ELF file on Linux, or a Mach-O file on MacOS. 

If you want to share a Rust program, you simply share the executable itself. Of course, you might also share the source code, so that the other person can inspect the code and/or compile it for whichever computer architecture (Windows, Linux, MacOS, or pretty much anything else) they wish.

In practice, the difference is quite small. In both R and Rust, you write source code, and then give that source code to another program to handle. For R, you'd invoke `R mycode.R` or `Rscript mycode.R` (or, more likely, press the Play button in RStudio). For Rust, you invoke `cargo run`. The end result is the same: output to the terminal.

One important difference is that you cannot directly step through your code line-by-line in Rust as you can in R. Thankfully, Rust has a lot of features that helps you reason about what your code is doing at any given moment without actually having to run it (so-called *static analysis*).

## Rust is Strictly Typed

You've probably encountered issues with *types* in R before. For instance, you might accidentally give a string to the `mean()` function, which produces an error. However, how do you know what type of object you have at hand? In R, there are a few helper functions such as `class()` and `typeof()`, but generally speaking, you're just supposed to know. It's obvious for things such as strings (which, rationally, are not numbers), but how about for the result of a statistical modeling function - what is the type of that?

R is a *weakly*[^1] typed language, meaning that types are not very set-in-stone. In fact, you can simply change the type of any object in your R script whenever you want with something like `class(x) <- MyNewType` - it is simply metadata.

To illustrate, here is the function signature (the input and output row) of the base R `mean()` function as written in the R source code:

```r
mean(x, ...)
```

What type can `x` be? Further, what type(s) can `...` be? Or what type does the function return? We simply do not know from looking at this line. An R programmer would presumably read further in the documentation and hope that 1) there is a description of the arguments, and 2) that the description is up-to-date.

To contrast, Rust is a *strictly* typed language. Here is a hypothetical mean function (since Rust does not have one built-in):

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:example_mean}}
```

The function signature directly tells us what goes in and what comes out: `x` is of type `Vec<u32>`, and whatever is returned from `mean()` is of type `i32`. This gives us a guarantee on what goes in to and what comes out of any given function - surely, this is a good thing for scientific reproducibility.

In fact, you *cannot* write Rust in a way that makes types implicit. Try running the following code snippet in a clean Cargo environment (pick a directory, type `cargo new examplecode`, and change the code inside the generated `/src/main.rs` to the following):

```rust,ignore
fn main() {
    fn mean(x) {
        return 0;
    }
}
```

You should get the following compiler error:

```
   Compiling examplecode v0.0.1 (/examplecode)
error: expected one of `:`, `@`, or `|`, found `)`
 --> src/main.rs:2:14
  |
2 |     fn mean(x) {
  |              ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
2 |     fn mean(x: TypeName) {
  |              ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
2 |     fn mean(_: x) {
  |             ++

error[E0308]: mismatched types
 --> src/main.rs:3:16
  |
2 |     fn mean(x) {
  |               - help: try adding a return type: `-> i32`
3 |         return 0;
  |                ^ expected `()`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to 2 previous errors
```

There are two errors here: the first one tells us that the compiler expected a type annotation after the `x` parameter, but we gave none. This simply shows that we have to explicitly state what type each parameter is, otherwise the compiler is angry with us and does not let us compile the executable.

The second one tells us that the return type and what is actually returned is mismatched - we tried to return an integer[^2], but the expected return type was `()`. What's going on here?

In Rust, every function has a return type, whether there is an explicit return or not. If left out, that return type is the *unit type* `()`. There is no real equivalent to this type in R - `NA` comes closest, but as we will see later, Rust has a much more powerful way to express missing values.

The compiler checked through our code, and saw that we wanted to return an integer. However, the function signature implied a unit type to return, so the types are mismatched - hence the error.

At the start of your Rust journey, this strict typing will most definitely be a headache. However, I suggest you stick with it as much as possible. For instance: note, that we never needed to know *what* the function does in order to understand its input and output. This is a core feature of Rust's type system: you only need the function signature to understand the function, and the implementation (what the function does) is left to the more advanced programmer or the developer themselves.

[^1]: Some people argue that "weak" is derogatory in this context, but I believe it is descriptive. Weak typing is not somehow "lesser" to strong or strict typing, and there are many scenarios in which weak typing is preferred.

[^2]: The compiler cannot actually even tell whether we tried to return a signed or unsigned integer, nor what bit length the integer was, since there is no additional information to distinguish any of the integer types.

## Sidenote: Function Purity {.advanced}

There is a caveat to the previous statement about how the function signature is supposed to guarantee the readability of the function itself, and that is *mutable parameters*.

Consider the following function signature:
```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:example_mut_fn}}
```

The type signature `&mut Vec<u32>` tells us that we don't give the function a vector of unsigned integers, but instead the memory address of such a vector, with the additional allowance that the function may *change* the contents at that memory address. We will go through references and pointers later, so just understand that we are allowing the function to mutate something that exists outside of the function body itself.

This is called an *impure* function, because it changes the state outside of its call site. Consider the following usage of such a function:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:example_mutability}}
```

On first glance, you would expect `_res_1` and `_res_2` to be equal, right? However, since they both take mutable references to `data`, it is very possible (and even likely) that the state of `data` changes between the first and second function call. In other words, what goes in the first time might not be what goes in the second time.

This makes reasoning about the code more difficult, since now you *have* to know what the function actually does. With a mutable argument, it is reasonable to expect that the input is somehow changed in the function[^3].

While mutability absolutely has its place, and is sometimes practically necessary to accomplish a given task, it can be a problem regarding debuggability and actually make scientific reproducibility a bit harder to accomplish. Therefore, I recommend that you stick with immutable data as far as possible, even if it means writing more verbose code or "throwing out" a lot of duplicates.

Note, that this argument applies mainly for statistical analysis aiming at reproducibility. If you are aiming for fast code or small binary sizes, you should probably learn to use mutability in a smart way - those things are, however, not a practical concern when you're writing an analysis for a research paper.

[^3]: On standard warning levels, the compiler actually warns you if something is made mutable but never mutated. **If you do not need mutability, you should always opt out**: instead of `let mut x = 5;`, write `let x = 5;` if `x` never needs to change.

## Some Major Syntactic Differences

There are a few syntactic differences that are valuable to keep in mind when writing Rust code. They can be summarized by comparing this R code:

```r
x <- 5
```

to this Rust code:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:let_example}}
```

1.  Variable definitions are done with `=`, not the arrows `<-` or `->`.
2.  Assignment is done with the `let` keyword, followed by the variable name.
3.  Almost all code lines end in a semicolon `;`.

Finally, and perhaps most importantly: errors are actually informative in Rust, as opposed to cryptic and geared towards computer engineers in R. One key feature of becoming a Rust developer is learning to love the compiler: it tells you 1) what went wrong, 2) where it went wrong in the code, and 3) what are some usual ways to solve the issue. It even knows simple spelling mistakes!

There are, naturally, many more differences between R and Rust, but these are the ones I found most salient when switching languages, and which took me a fair bit of work to correct in my automatic behaviour.

## Rust Types, A Very Quick Overview

By now, it should be dawning on you that Rust's type system can be a benefit when writing reproducible statistical analyses. However, it is also very vast and complicated. This book is not meant to teach you an advanced level of Rust - only to help you get started writing statistical analyses in Rust instead of R.

To do this, however, we need to quickly go through the main data types you will encounter. More information is available in the [Rust book](https://doc.rust-lang.org/book/) or the [standard library](https://doc.rust-lang.org/std/) documentation.

### Strings

In R, text is represented with the `character` type. In Rust, there are a *lot* of different types depending on the use case. The most important ones are `String` and `str`, which are both - confusingly - pronounced "string".

The `String` string is the most commonly used type for representing text data, and what you probably should reach for in most circumstances:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:example_string_manipulation}}
```

`String`s are distinguished from another string type, `str` or the string slice. In the above example, the literal string `"Hello!"` is an `str`, while the result of `String::from("Hello")` is a `String`. The difference is rather technical and has to do with how the text is laid out in memory.

In most cases, creating `String`s from `str` is enough to get you started with managing textual data. If you need to concatenate two strings, note that the left-hand side *must* be an owned `String` (see below on references and ownership).

### Numbers

In R, numbers come in one basic variety: `numeric`. In Rust, there are once again a *lot* of different varieties. The main varieties are **unsigned integers**, **signed integers**, and **floating-point numbers** or floats.

Each of the number varieties also comes in several different *bit lengths*. The bit length determines the maximum length of the number, or, for floats, the maximum precision of the decimal number:

| Unsigned | Signed | Float |
| -- | -- | -- |
| u8 | i8 | -- |
| u16 | i16 | -- |
| u32 | i32 | f32 |
| u64 | i64 | f64 |
| u128 | i128 | -- |
| usize | isize | -- |

Unsigned integers have no sign, i.e., they start from zero and grow in the positive direction. Signed integers, on the other hand, have a signed, so they grow in either direction around zero (positive and negative). Floats can be negative.

However, because both unsigned and signed integers have the same maximum size, unsigned integers are always capable of growing "larger" than signed integers. For instance, the maximum range of `u8` is all the possible values that eight bits can represents, i.e., `0..2⁸`, or from zero to 255[^4]. However, the maximum range of `i8` is the same number of values, but centred around zero, i.e., `-128..128` or from negative 128 to positive 127.

We can do all customary operations on the number types: addition, subtraction, multiplication, division, and modular division (the `%` operator). However, it is important to note whether the result can fit in a given data type. For instance, while it is syntactically completely fine to divide `5 / 3`, without specifying that you want a float, you will get `-1`, i.e., the result with the decimal part dropped. The same applies for values that might become bigger than the original data types allow: the operation `255u8 + 1` will result in an *integer overflow* unless you explicitly ask for a `u16` or larger, since `u8` cannot fit numbers greater than 255.

[^4]: In programming, it is customary to start counting from zero. The first two-hundred-and-fifty-six values, including zero, thus are from 0 to 255.

### Booleans

Booleans are pretty much the same in R and Rust. Whereas R uses the syntax `TRUE` and `FALSE` (all uppercase), Rust uses `true` and `false` (all lowercase).

However, one key difference between the two languages is that Rust will *never* implicitly convert a boolean value to an integer. In R, the following statement is valid and evaluates to the value `2`:

```
x <- true + true + false
```

However, the same in Rust will lead to a compiler error:

```rust,ignore
let x = true + true + false;
```

```
error[E0369]: cannot add `bool` to `bool`
  --> source_file:1:18
   |
1  |     let x = true + true + false;
   |             ---- ^ ---- bool
   |             |
   |             bool
```

However, you *can* convert booleans explicitly using the `as` keyword - more on that later.

### Structs

While you may or may not have come across it, R actually has a way to create objects in the same vein as other object-oriented programming languages. These are called *classes*, and there are numerous flavours of them, such as S3 and R4.

Neither R nor Rust are particularly object-oriented programming languages, which means that their usage of objects/classes is more pragmatic than dogmatic. However, Rust has a very important type as its class structure, called the **struct**.

Structs come in a few different varieties:

```rust,
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:struct_examples}}
```

The most common version you will use is a struct with one or more **fields**. Fields can be named anything you want and must be type-defined, like how the `FilledStruct.field` example above is of type `u32`. You can mix and match types between fields freely, but one field may only be a single type. Also, as can be seen in the example, a struct may have zero fields.

A tuple struct is similar to a regular struct, except that the fields are unnamed. Instead of referring to the fields by name, you refer to them by index: in the above example, `TupleStruct.0` refers to the `f32`-typed field, and `TupleStruct.2` to the `f64`-typed field.

Finally, there are the unit structs, that contain no data. These are used less commonly, but may be useful if you want specific marker objects, i.e., objects that do not contain any specific data.

### Enums

If you ask a Rust developer what changed their life the most when switching to Rust from \<insert_programming_language_here\>, chances are that they will name enums and pattern matching - two highly related concepts.

R has no equivalent of the Rust **enum**. However, you can easily conceptualise it as an everyday list with a limited number of options. The name comes from "enumeration", meaning listing:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:enum_examples}}
```

Similar to structs, enums come in multiple varieties. At its simplest, an enum can be empty, though it is more useful if filled with something.

Enums can be simple, where their **variants** do not contain any values. This might sound unnecessary, but think of it this way: you can codify exacly which options are available through a simple enum. Say for instance, that you have a statistical model that allows the user to choose between, say, a frequentist estimator and a Bayesian estimator. You *could* have the user give a `String` with their choice, but that is very error-prone. Similarly, an integer representing the method is difficult to parse and not immediately obvious to the user. However, an enum would be extremely obvious:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:model_enum}}
```

Now, the user has two - and only two - options: the frequentist or the Bayesian estimator. If you later extend the function to cover a third, hitherto unforeseen and mysterious method of statistical estimation, giving users the option is as easy as adding one line of code - just add another enum variant!

### Optional values

R has a specific data type for representing missing values, `NA`. Rust also has one, though it is slightly more complex: the option type `Option<T>`. The value `T` stands for any other type: an option can either be `Some(T)` or `None`, i.e., something of value, or nothing.

In fact, under the hood, the option type is nothing but an enum:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:option_def}}
```

Options are most often used through pattern matching against the different, well, options:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:matching_options}}
```

Options also have several helper methods to match against their state and either return something or do something else. For instance, the methods `Option.is_some()` and `Option.is_none()` return booleans representing whether the option contained a value or not. A full listing of all methods can be found in the [standard library documentation](https://doc.rust-lang.org/std/option/enum.Option.html).

There is a special method that you should generally **avoid** using: `Option.unwrap()`. If the value is `Some(T)`, this will return `T` (i.e., unwrap the wrapped type). However, if the value is `None`, this will *crash the program*. You can probably see why this should only be used if you're certain that the value is `Some(T)` - and if that is the case, why use an `Option(T)` when `T` is enough?

### Error handling with Result

R has no clean way of handling errors, pure and simple. In R, the most common way of handling an error is to abort the script.

In Rust, we take a very different approach: most often, 'error' just means that you received an unexpected value, but that the situation can still be managed somehow. For this, there is the `Result<T, E>` type:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:result_def}}
```

In `Option<T>`, we only had one wrapped type inside the `Some(T)` variant, but in `Result<T, E>`, we have both a wrapped success value in `Ok(T)` and a wrapped error value in `Err(E)`.

Most often, you want to have the error type be a very specific kind of type - more on that later. However, technically, the error type can be anything - a numeric, a `String`, a struct, or even a unit type.

Like Options, Results can also be pattern-matched, and the existence of the two variants can be checked with the methods `Result.is_ok()` and `Result.is_err()`. More useful methods are, as usual, documented in the [standard library](https://doc.rust-lang.org/std/result/enum.Result.html).

### The unit type

In Rust, there is one type that bears no semantic meaning: the unit type `()`. It always has the same value, `()`. Yes, thinking about the value of nothing make little sense.

The unit type is most useful if you define functions that are only ran for their side effects, i.e., should not return anything meaningful. For instance, if the function is *fallible* (i.e., can fail in some way), you can express this by returning a `Result<(), Error>`, where `Error` is some specification of the failure while a success is represented by the unit type.

## Making Things Go: Functions, Methods, and Traits

Programming can be boiled down to two things[^5]:

1.  Representing things
2.  Making things do something

Thus far, we've looked at how we can represent things in Rust, and how that differs from R. It's time to look at how to make things do something.

While it's hard to pin down a simple description on either R or Rust, both languages lean heavily into what is called **functional programming**. This type of programming focuses heavily on **functions**, i.e., recipes to create action. A function may be as simple as `return true`, or it may span hundreds of lines.

This section will start with functions, and then delve deeper to explore the several ways in which Rust makes things go.

### Functions

In R, a function is **declared** with the following syntax:

```r
x <- function(a, b) {
    return
}
```

Compare the Rust varieties:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:basic_fun}}
```

Okay, that might look *quite* a bit more complex, but we'll dissect it piece by piece.

A function is declared using the `fn` keyword, followed by the **identifier** that the function is bound to. The function will subsequently be called by using the identifier, so this is similar to binding the function to the variable name in R. In our case, the function is called `x`.

Function **parameters**[^6] come in the parentheses. In R, we simply list the names we want to use inside the function body, but in Rust, we also have to define their types. In our case, the function takes two parameters, `a` and `b`, the former of which is an `i32` and the latter a boolean.

In R, you don't declare the function return type at all. In Rust, you have two options: leave it out, implicitly declaring it as the unit type, or explicitly declare the return type. The return type is declared by an arrow `->` after the parameters, and then the type. In our case, our function `x` returns a Result of either the successful unit type or an `i32` error type.

> [!TIP]
> You have probably noticed by now that a lot of the examples contain variable names with an underscore `_` in front of it (such as the arguments `_a` and `_b` in the previous example). This is because the Rust compiler is very, *very* helpful - so helpful, that when compiling a function whose arguments are never used in the body of the function, it will warn you. Those warnings can be suppressed by prepending an underscore to the variable name. However, in real code, you should probably not do this, as if the argument is never used, it should not be there in the first place.

Once a function is declared, it can be used. In R, usage looks something like this:

```r
result <- x(5, TRUE)
```

And in Rust, the equivalent is almost identical:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:fun_usage}}
```

### Methods

It's all fine and dandy with free-floating functions, but what if we want to restrict functions to only pertain to certain types of objects? For instance, we might have a statistical model function that returns a custom object representing the result of the model, and we would like to have another function that extracts the model fit indices from that model.

We *could* write a function whose argument is restricted to the model object type:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:naive_method}}
```

However, there is a more elegant and idiomatic way to write this in Rust, using **method implementations**:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:method_ex}}
```

> [!NOTE]
> "What's that ampersand doing in the function parameters?" Don't worry, we will get to that later!

An implementation block contains zero or more **methods**, i.e., functions that belong to a given type. It makes sense for the fit index extraction function to belong to the model result object, since it acts on the results and nothing else.

There is another important difference in the parameters: the function took a parameter named `_model: &MyModelResults`, while the method took a parameter `&self`. This is because the method knows who its owner/parent is, and can access that through the `self` keyword. In other words, `self` is a stand-in for the object that we are operating upon, `MyModelResults`. For functions, there is no equivalent, because free functions are not owned (sired?) by anything.

The difference becomes even more clear in how usage works:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:using_methods}}
```

Functions are used as usual, but methods are used by **accessing** the method on the object with the dot operator `.`. Since we created a `MyModelResults` object called `model_result`, we have to access the method on that object, i.e., `model_result.extract_fit_indices()`.

Before we continue, there is one important pattern to learn: **method chaining**. Since methods operate on the object that owns them, and since methods are just functions that take input and return output, it stands to reason that they should be able to *return their owners*. This is exactly what methods allow:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:method_chaining}}
```

We first create an object `MyObject` and implement two methods for it, `first_method` and `second_method`. Both of them take the object as input, and return the object as output. Finally, on the last two lines, we create a new instance of `MyObject` called `x`, and then call the methods one after another, akin to a pipeline.

This pattern makes a lot of sense in statistical work, where we often do multiple transformations on the same dataset, one after another. Since the dataset itself does not change in meaningful ways - it remains a "dataset", regardless of its content - it makes sense that methods implemented for a dataset would be able to simply return the dataset after their transformations. The chain would naturally end once we've completed all data transformations and move over to conducting some type of modelling.

However, what if the chain does not need to end? What if there was a way to describe not how certain objects behave, but behaviour itself?

### Traits

Before we start, congratulations on getting this far! We are doing a crash course in Rust programming, and things are about to get very meta - and we are definitely far from what R can accomplish now.

We've looked at how objects can be defined, and how behaviour can be defined. However, one thing might stand out for you: there exists behaviour that is valid for a group of objects, not just a single object type, yet Rust seems to restrict functions and methods to singular data types.

For instance, consider our previous example with extracting model fit indices. Sure, it applies for our custom statistical model, but the behaviour of extracting fit indices also makes sense for a bunch of other statistical models.

In Rust, we can codify behavioural **traits**:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:trait_examples}}
```

Instead of implementing the method directly on `MyModelObject`, we instead define that `MyModelObject` has the `Model` trait. We then tell the compiler what `extract_fit_indices` should do on `MyModelObject`. The difference is that now, the function is not directly tied to *our* model, but instead able to be defined for **any model**.

Now, either we or some other person could come around and write implementations for other statistical modelling - all while using the exact same function signature. This means that we can rely on the promised function signature in the `Model` trait for any object that implements `Model`, regardless of *how* it implements it. We don't care *how* to extract fit indices from different statistical models, only that it is possible and that the output makes sense.

Traits are absolutely everywhere in the standard library:

-   Want to print an object to the screen? Define its `Display` implementation.
-   Want to derive default values for an object? Define its `Default` implementation.
-   Want to compare equality of two objects? Define the `PartialEq` implementations for both objects[^7].

One final remark: Some traits can be automatically implemented. If you've looked through the standard library documentation, or even looked at some Rust source code, you may have stumbled upon single lines of code in the form of `#[something]` right above functions, objects, and traits. These are called **attribute-like macros**, and of them, the Derive macro allows you to automatically implement some of the most standard traits, like `PartialEq`, `Clone`, and `Debug`. Simply list them one after another inside a macro in front of your object:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:derive_macro}}
```

Because we derived `Debug`, we can use the `dbg!()` macro on `MyModelResults` and get sensible output. The derivation of `Clone` allows us to duplicate the value of `x` into `y`. Finally, the `PartialEq` derivation allows us to compare `x == y`.

Note, however, that deriving a trait means that you opt in for an automatic implementation - whatever that implementation might be. If you want to have control of exactly *what it means* to, say, compare your object with itself, you should implement the trait manually. In most cases, the derived implementation is sensible, but not always.

[^5]: And only two things. If you don't believe me, search Wikipedia for "category theory" and be amazed/confused.

[^6]: The inputs to a function are called "parameters" in the function definition, while the actual inputs given to a function when it is called are called "arguments". Why the distinction? I don't know.

[^7]: For a trait concerning two objects like `PartialEq`, the reality is a bit more complex. In particular, implementing `impl PartialEq for A` will automatically allow you to compare the type `A` with itself. In order to compare it with another type `B`, you must implement `impl PartialEq<B> for A`, which will give you comparisons like `A == B`. Further, in order to reverse the order, you also need `impl PartialEq<A> for B`. Read more in the [standard library documentation](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html).

## Describe the House, or Just Say the Address: References

It is time to address the elephant in the house: the recurring ampersands. These tie deeply into the way Rust manages memory for you, and the way Rust makes sure that you don't mess up the memory that your program is using.

Rust has a concept of **ownership**. At its core, it means that values are owned by something. For instance, when you declare a variable with a value, that variable becomes the owner of the value.

In a lot of cases, and in a lot of programming languages, ownership does not really matter, as either the data that is moved around is light enough to be **passed by value**, or the program does not need to concern itself with such trivial things as speed or efficiency. Passing by value simply means that when a value is given from some owner to another (e.g., passed as an argument to a function), everything that exists in memory is moved into another location.

This moving can, however, be quite expensive - especially if done, say, once a function call over five million rows of data. Another option is to **pass by reference** where, instead of moving the memory contents, the owner simply tells the receiver the **memory address**. A memory address is very often guaranteed to be much lighter (it might be only a couple of bytes long), and the receiver can still access the underlying data.

Rust handles this mainly with **references**. A reference is created with the ampersand `&` operator, and a reference is **dereferenced**, i.e., turned back from an address into its value, by the dereference `*` operator:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:ref_examples}}
```

In the above example, I have added **type annotations** to clarify what is being stored in each variable. In most cases, the Rust compiler can infer the type from the surrounding code (e.g., from a function signature), which is why we have not used them before.

We first set `x` to be the `i32` value of 5. We then set `y` to be a *reference* to `x`. This means that `y` actually stores the memory address of `x`, not the value 5 itself. Finally, we set `z` to be the *dereference* of `y`, meaning that we retrieve the data that is stored at the memory location stored in `y`.

In previous examples, every time we have used the `&` operator, we have actually dealt with memory addresses, not actual data. For instance, in non-chainable methods, it is common to take references to the object the method belongs to through `&self` instead of the object (`self`), in order to make sure that the ownership of the object does not pass to the method that is calling it. If that were the case, once the method has been called, the previous owner would lose ownership, and because the method scope is destroyed once it is called (see below for an explanation of scope), the object would be functionally lost to us.

Since a reference to an object does not transfer ownership of said object, we can create an unlimited amount of references. However, this leads us to one of the most pernicious bugs in all of programming: use-after-free.

## Give the House Address, and Permission to Paint the Walls: Mutability

A **use-after-free** bug occurs when two objects have access to the same memory location. The first object decides to delete the data at the memory address, signalling to the operating system that the address is free to use for other programs. However, after this deletion, the second object tries to access the data. The operating system sees this, says "Absolutely not!", and condemns the program for a memory access violation - killing the program immediately. Anyone who has ever programmed in C or such systems language is intimately aware with this event.

Thankfully, Rust makes this (almost) impossible, through the concept of **mutability**. Any variable or object may exist in one of two modes: immutable or mutable. By default, when you declare a variable with the `let` keyword, it is immutable, meaning that it cannot be changed *at all* after declaration.

To make a variable mutable, you simply insert the `mut` keyword:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:mut_examples}}
```

Because of the `mut` keyword, the compiler now allows us to reassign the value in `x` from 5 to 7 on the second line. The third line does an **add-assignment** with `+=`, which is the same as writing `x = x + 2`, so the final value of `x` is 9. Note, that reassignments do not use the `let` keyword.

This keyword is also used when declaring references; references can be either **immutable** (`&data`) or **mutable** (`&mut data`).

The use-after-free issue is resolved when we look at how Rust handles references to mutable and immutable data:

| State | Immutable references | Mutable references |
| -- | -- | -- |
| Immutable variable | Unlimited amount | None |
| Mutable variable | Unlimited amount, if no mutable reference | One, if no immutable references |

In other words, Rust allows *either* an unlimited amount of immutable references, *or* exactly one mutable references, but never both. In order to use a mutable reference, the variable itself has to be declared as mutable - this shows other users that the variable may change at some point.

Consider these two almost-equal trait methods:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:mut_ref}}
```

We do not know what these methods do. In fact, we *cannot* know, because it is up to the implementing object to define exactly what happens. However, the first method uses an immutable reference to `self`, while the second method uses a mutable reference to `self`. The compiler guarantees to us that the first method cannot change the object in any way, meaning that we are free to reference the object in other situations. We can even reference it multiple times at the same time, since the referencing rules guarantee that there cannot be a mutable reference while there is at least one immutable reference. However, for the second method, we cannot guarantee such a thing: it may be that the method does nothing to the object, or it may completely change its underlying data. The compiler blocks us from reading any data by reference as long as this mutable reference is alive (more on that soon).

Thus, the use-after-free issue is resolved! However, one final question remains: how does the compiler know when a reference is being used?

## I Live in the Here and Now: Scopes, Lifetimes, and Namespaces 

You have made it to the last part of this admittedly very long chapter! We are now going to tackle some of the most difficult parts of Rust, but first, a slight but related detour into scoping.

### Scopes

Most languages have some idea of scoping. In R, functions have **inner scope**, so that a variable declared inside a function cannot be accessed outside of the function. Consider the following R code:

```r
x <- 3
myfun <- function() {
    x <- 5
}
print(x)
```

What will be printed on the final line? Because of scoping rules, the print statement only sees the outer `x` (declared on the first line), not the `x` inside the function, so the statement prints the value of 3.

This is also called **variable shadowing**: if you were to access `x` *inside* the function, it would evaluate to 5. In other words, the inner `x` is said to shadow the outer `x` due to the same name.

Rust has a similar system, but - and you should know this by now - Rust dials it up to eleven. Firstly, Rust manages scope through the use of **blocks**, demarcated by curly braces `{}`. Consider the following code:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:scope_examples}}
```

Like R, Rust allows for variable shadowing wherever the scopes of two variables are distinctly separated. In this case, the inner declaration of `x` is only valid for the inner scope, and so never clashes with any usage of `x` in the outer scope. The final line prints 5, since the inner `x` is not available to the outer scope. If we were to move the `println!()` macro invocation to the inner scope after the variable declaration, it would print 7.

However, if we were to move the macro to the inner scope but *before* the variable declaration, what would happen? In that case, the macro would find the `x` that was declared in the outer scope, since nothing shadows it, and would print 5.

Let's now move over to lifetimes, another important but often quite difficult-to-grasp aspect of how Rust guarantees these memory safety behaviours we have discussed.

### Lifetimes

Every object in Rust has a lifetime: it is born, it lives, and it dies. So it goes.

In some programming languages, the creation of an object is defined through its constructor method. Rust does not have such a pattern directly, as instead objects are created through instantiation - something called "**resource allocation is initialization**", or RAII for short.

Once an object is alive, it can interact with other objects. We can read it, we can write to it (if declared mutably), we can print it to the screen, and other fun stuff we could come up with.

Finally, an object dies when it goes **out of scope**, i.e., the scope in which the owner lived ends. At the point of death, the memory the object allocated is freed; the program tells the operating system that the memory is no longer necessary, and that it may be used for other things. At this point, it is obviously no longer valid to refer to that memory location from within the program, as we no longer should have access to it. Imagine moving out of your apartment but keeping the keys and still occasionally visiting!

All of this is tracked through the **lifetime** system. For the most part, this is implicitly done by the compiler, and you do not have to worry about it. However, it may sometimes become important to manually restrict lifetimes.

Consider the following function:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:lifetime_example}}
```

The function takes two string references, compares the length of the two, and outputs the *reference* of the longer string. Since either `x` or `y` could potentially be the longer one, either reference must be guaranteed to live equally long.

This guarantee is accomplished by telling the compiler this restriction through the use of **lifetime annotations**, which are textual tags that start with an apostrophe `'text`. In the example, the annotation `'a` tells us three things:

1.  `x` lives to at least time `'a`.
2.  `y` lives to at least time `'a`.
3.  The output result lives to at least time `'a`.

In other words, the result is guaranteed to be alive at least as long as both of the inputs. Note, that as soon as either `x` or `y` dies, the result is allowed to die as well, barring any other lifetime or scope constraints elsewhere.

This might be quite complex to grasp, and that's okay. The important thing to know is that lifetimes allow the compiler to give us structural guarantees that our data is safe to read and/or modify. As long as it compiles, it is safe - if it's unsafe, it does not compile[^8].

[^8]: Unless, of course, it is `unsafe`, in which case the compiler does not check for these guarantees. Unsafe Rust is its own, somewhat advanced field, which is mostly unnecessary if you intend to only work with statistical data analysis. In unsafe Rust, we can tell the compiler to relax and let us handle memory safety ourselves, which allows for some major trickery and speed increases but also let's us shoot ourselves in the feet if we aren't careful.

### Namespaces

Finally, to cap things off, we will consider a particular type of scoping called **namespacing**. This is mostly relevant when packaging your own crates, sectioning off code into sub-sections of your codebase, or when using code made by others.

In the starting chapter, we talked about how Rust has **crates** where R has packages. This is the main unit of code execution in Rust: whenever you run most Cargo commands, you are operating on a crate. At its simplest, the crate is the directory created by `cargo new` or `cargo init`, containing your source code and a `Cargo.toml` crate definition file.

However, crates can be further subdivided into **modules** using the `mod` keyword. This allows you to declare a **namespace**, so that similarly named objects can co-exist within one crate.

By default, anything inside a crate or module is private to that scope, meaning that the object is not available to either a parent or child scope. If we define a function in our crate, it cannot be accessed by another crate - unless we use the `pub` keyword to explicitly make it public.

Consider the following code:

```rust
{{#include ../examples/ex_from_r_to_rust/src/lib.rs:modules_example}}
```

In the snippet, we declare a module called `inner`. Inside it, we declare two objects: a constant value called `X` equal to the `i32` value of 5, and a function called `myfun()` that returns the `i32` value of 5. We further declare both to be public using the `pub` keyword, so that the parent scope can see and use them.

> [!TIP]
> "Why `const` instead of `let`?" Because Rust prefers to have variables only where something can, well, vary. Rust intends that all expressions are encapsulated in objects of some type, whether they are functions or simple calculations. While many other languages have the concept of "global variables", Rust prefers to use **global constants**, i.e., values that definitely, 100 percent, absolutely never can change during the runtime of the program.

> `const` values should be written in all upper-case to distinguish them from regular variables.

In the outer scope, we can assign the value of `X` to a variable called `x` by using the namespacing operator `::`, as in `inner::X`. The namespacing starts from the current scope, so that the first sub-level is `crate::first`, the second is `crate::first::second`, and so on.

We then go on to define a function called `myfun()`, which returns the `i32` value of 7. Note, that despite having the same name as the function defined in the module, it is nonetheless valid. This is because the module scope guarantees that the module's `myfun()` is only accessible through namespacing `inner::myfun()`, or in other words, they are distinct objects despite a surface-level similarity.

There are two keywords that can be used beside `mod`:

-   `super` lets you namespace objects from the parent scope, if one exists. Note, that you cannot go "further up" than the crate itself, as to the compiler, nothing exists outside of the crate.
-   `crate` lets you namespace relative to the crate root. This is useful especially if your module structure gets convoluted, and you want a quick way to find items in one module tree branch from deep within another branch.

Finally, as an alternative to the visibility keyword `pub`, there is also `pub (crate)`. This makes the item visible throughout the crate in which it was defined, but not anywhere else. In other words, you can make a particular object usable throughout your own code, but block it from being used by other programmers using your code in their own code.

## Finally...

Congratulations, you made it to the end! While you might not be a Rust engineer just yet, you should definitely have the prerequisites to start writing quite complex Rust code. There are still many more details in Rust to learn, but I hope you have begun to understand the power of this language for reproducible data analysis.

The next chapter will put these knowledges to practice, and give a guided exercise tailored towards statistical data analysis: we will build our own simple statistics library!

