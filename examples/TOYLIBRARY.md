# Source code for the toy statistical library in Statistics in Rust

The source code is divided into crates according to each subchapter in the book chapter, except for the "Setup" subchapter which contains no compiled code. E.g., "Starting Out" is in crate "ex_library_1", "Testing Our Functions" is in crate "ex_library_2", and so on.

Within each crate, each code block in the book corresponds with a dedicated module, numbered `vn` where `n` starts from one. This is to allow each separate code block to successfully compile in one go, instead of having to create a new crate for every version.

Unfortunately, this means that editing the text and/or the code might require:

1.  Changing the module numbering within a given sub-chapter and updating the numbering across *all* `#include` blocks in the text.
2.  Changing the crate numbering, editing both the workspace `Cargo.toml` and the `#include` blocks in one **or more** chapters, depending on which chapter(s) are affected by the numbering change.

If you decide to edit this chapter, **please be thorough and careful with your changes**. Make sure to build the book, and read through all of your suggested changes in both source text and code to confirm that cross-references are correct. Remember to check whether the numbering change affected any previously so-numbered chapters, and if so, fix their cross-references as well.