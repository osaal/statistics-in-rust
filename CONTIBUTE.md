# Contributing to Statistics in Rust

You are more than welcome to contribute to the book by correcting mistakes, expanding on the already written text, or even rewriting parts of the text yourself!

However, please make sure to follow the points laid out here. There is a simple checklist that you can check to make sure that you're following the code of contribution:

-   [ ] Examples are contained and tested.
-   [ ] The version number is bumped accordingly.

Please read down below what each requirement means.

## Code examples

Code examples are supposed to be self-contained in the `/examples` directory, and referenced in the book through the `#include` pre-processor directive. This makes sure that there is a single source of truth for the code itself, and allows for checking that the code compiles and runs.

The directory structure of `/examples` is as follows:

```
examples
├── chapter
│   ├── src
│   │   ├── lib.rs
│   │   ├── tests.rs
│   │   └── common.rs (optional)
│   └── Cargo.toml
└── Cargo.toml
```

The `/examples` folder itself is a Cargo workspace.

> ![WARNING]
> Remember to add every new example project related to a chapter to the workspace members list in `/examples/Cargo.toml`.

Every chapter gets its own Cargo project in that workspace. To easily make a new one, you can copy an old chapter's examples directory and change the defaults in the Cargo.toml to fit.

Each chapter should have at least two files: a `lib.rs` (where the examples live), and a `tests.rs` (where tests live).

Every example should be written as a stand-along function, with the following function signature:

```rust
#[inline(always)]
#[allow(dead_code)]
fn example_function() -> Result<(), ErrorType> {
    // initialization code goes here...
    // ANCHOR: example_function
    // visible code goes here...
    // ANCHOR_END: example_function
    // finalizing code goes here...
    Ok(())
}
```
The error type can be defined wherever, as long as it is specific to the functions - every function returns one, and only one, error type, allowing automated grepping and debugging in the future.

The two directives make sure that the code is actually added to wherever it is ran, and also satisfies `rust-analyzer` when it inevitably complains about a lack of function use (this is okay due to the testing suite).

> ![WARNING]
> The examples should be *fully* pure, meaning that any and all imports should happen **inside the function body**.

When reusing code snippers in the book itself, use the anchor syntax in the `#include` directive:

```ignore
{{#include /relative/path/to/example.rs:example_function}}
```

You may choose to split every example into its own function, or combine multiple dependent examples in one function using different anchors.

> ![WARNING]
> Do not use row number indexing in `#include`, since this will break the moment someone touches the code.

Tests should simply call the function and check the returned `Result.is_ok()`:

```rust,ignore
use crate::*;
#[test]
fn test_example_function() {
    assert!(example_function().is_ok())
}
```

This makes sure that each function is actually ran and not just compiled, which will catch simple runtime errors due to changing dependencies (or broken code).

## Version numbering

This project uses version numbering inspired by, but not beholden to, semantic versioning. The basic rules are:

1.  Minor grammar corrections, basic code fixes to make examples functional (where the example function signature stays the same), other such maintenance? --> Increment patch version (X.Y.n -> X.Y.n+1).
2.  Changes in chapter structure, dependency updates that break the dependencies' API (usually major version shifts), new chapters? --> Increment minor version (X.n.Y -> X.n+1.0). Note the resetting of the patch version!
3.  Larger changes than this? --> Consult maintainers, and we will discuss whether the update is large enough to require a major version incrementation (n.X.Y -> n+1.0.0).

Messing up the version numbering in your pull request is no big deal, as it is easy to fix!
