# Statistics in Rust

This is the source code for the online book "Statistics in Rust".

If you're only interested in reading the book, please visit its website (NYI).

This repository is for writing and development.

## Installation

The repository uses [*devenv*](https://devenv.sh) to manage a local Rust installation, to guarantee compilation across machines and system configurations.

Start by installing *devenv* onto your machine.

> [!IMPORTANT]
> The book uses the version of *devenv* in [nixpkgs:25.11](https://search.nixos.org/packages?channel=25.11&query=devenv&show=devenv), which is somewhat behind the bleeding edge. Currently, this means version 1.11.1 - which differs from the latest documentation, which is written for version 2.X.Y. The book will update once nixpkgs:26.05 is released.

You can additionally install [*direnv*](https://direnv.net/) to automatically switch to the development environment when moving into the directory. This is optional, but recommended for ease-of-use.

Clone the repository:

```shell
git clone https://github.com/osaal/statistics-in-rust
```
### With direnv

In a terminal, move into the cloned repository:

```shell
cd /path/to/repository
```

You might be prompted with a warning regarding the file `.envrc` being blocked. Review the file (in `/` in the repository) if you wish, and run `direnv allow` to approve of the file. See an explanation for this in the [devenv documentation](https://devenv.sh/integrations/direnv/#approving-and-loading-the-shell).

After this, moving in and out of the directory should trigger *devenv* to install the necessary Rust toolchain locally on the first attempt, and subsequently activating the environment on follow-up attempts. It should also add the necessary Nix overlay to declaratively install and manage Rust.

Test the installation with `cargo --version` or `which cargo`, making sure that Cargo is available.

### Without direnv

In a terminal, move into the cloned repository and activate the *devenv* shell:

```shell
cd /path/to/repository
devenv shell
```

On the first attempt, it will install the necessary Rust toolchain. After this is done, you should have a functioning Rust installation - you can test this by calling `cargo --version`, which should output the current version of Cargo.

## Structure

The book is divided into two major directories: `src` and `examples`.

### Text source `src`

The `src` directory contains the text chapters as Markdown files. Edit these when you need to change/add to text in some way.

### Example code source `examples`

The `examples` directory is a Cargo workspace containing all the Rust example code, as well as the project dependencies. Edit these when you need to change/add to the code examples.

Each `$NAME.md` chapter has a corresponding `/examples/$NAME` directory, representing a Cargo crated named `examples_$NAME`.

To build the source code of a single example, use:

```shell
cargo build -p examples_$NAME
```

and to build all examples (required for pull requests), use:

```shell
cargo build
```

## Seeing changes

Preview your changes:

```shell
mdbook serve --open
```

This opens a live-updating website view in your browser. Each time you save a Markdown document, it rebuilds the website.

## Contributing

Contributions require you to confirm that the `/examples` Cargo project builds on your machine, *regardless of if you edited the code or not*.

Call `cargo test` and `cargo build` in the directory and attach the output to your pull request.

Please note that **all** examples must be tested and built for a pull request, not just the one(s) you have touched (in case of dependency issues).

**Pull requests without `cargo test` and `cargo build` outputs will not be accepted.**
