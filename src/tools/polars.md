# Polars

```ignore
{{#include ../../examples/ex_polars/Cargo.toml:20:21}}
```

Polars is a crate for querying DataFrames, a high-speed rectangular data format suitable for both small and large datasets. It is available in multiple languages, but written originally in Rust.

Polars is extremely versatile and wide, with both default and generic implementations, so you can extend the crate with your own data containers if need be. However, for most use cases, the default containers are more than enough (and well-tested!).

## Basic usage

DataFrames can be created in numerous different ways. Here, we discuss the simpler options, and show how they can be used programmatically.

### An empty `DataFrame`

You can initialize an empty dataframe very simply:

```rust
{{#include ../../examples/ex_polars/src/lib.rs:empty_df}}
```

### The `df!` macro

The easiest way to construct a DataFrame from hard-coded data is to call the `df!()` macro:

```rust
{{#include ../../examples/ex_polars/src/lib.rs:df_macro}}
```

The macro itself takes a Rust expression as a name, followed by a fat arrow `=>`, followed by another expression. It generates code creating a DataFrame with the following structure:

```rust,ignore
let _ = DataFrame::new_infer_height(
    vec![
        Column::from(<Series as NamedFrom::<_, _>>::new(
            col_name,
            data,
        )),
        Column::from(<Series as NamedFrom::<_, _>>::new(
            col_name,
            data,
        )),
        { ... }
    ],
);
```

In other words: for each pair of name and data slice you give it in the macro, it creates a `Column` object. These objects store the actual data along with some potential metadata.
