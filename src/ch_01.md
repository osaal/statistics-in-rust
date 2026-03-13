# Getting Started

This chapter will help you get your dependencies set up.

## Rust and an editor

Doing statistical analysis in Rust will, naturally, require an installation of Rust.

You can follow the [official instructions](https://rust-lang.org/learn/get-started/) to install Rust on your machine.

Make sure to also install some sort of text editor, as instructed in the official instructions.
I recommend a fully fledged IDE (*integrated development environment*) for helpful tools such as auto-complete, instant error feedback,
and intelligent code suggestions, such as **VS Code** or **RustRover**.

> [!TIP]
> If you're coming from R, you might be accustomed to RStudio, the dedicated IDE for R programming.
> I recommend switching to a general IDE for Rust, such as VS Code, because as it turns out,
> you are able to also write and debug R code in VS Code.

> [!NOTE]
> This book is version-locked to a particular version of Rust. You can check the versioning from the source repository, from the file `rust-toolchain.toml`. Please make sure that your version of Rust has the same major version (the first number), and a minor version (the second number) greater than or equal to the book's version.

## Package prerequisites

We are going to be using a number of different add-ons called *crates*, as Rust does not ship with the necessary tools to conduct statistical analysis. For instance, we need to install a crate called [**Polars**](https://pola.rs/) for usable rectangular data structures.

In Rust, crates are offered by the central repository [crates.io](https://crates.io), similar to how R distributes packages through *CRAN* or Python through *PyPI*.

For Rust, adding a crate can be done in two simple ways: calling the Cargo package manager or editing the Cargo.toml configuration file after creating a project.

> [!TIP]
> If you forget to install a package, trying to use it in your Rust code will cause a compiler error. If you are using an IDE such as VS Code, you will be told that the package is not available in the namespace before even compiling.
