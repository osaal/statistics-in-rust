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

> [!WARNING]
> The `df!` macro requires each data vector to be the same length, so that there is no missing data in the resulting DataFrame.

### Read a CSV file

You can also easily create a DataFrame from an existing CSV file:

```rust
{{#include ../../examples/ex_polars/src/lib.rs:read_csv}}
```

1.  Create a new `CsvReadOptions` with the `Default` method.
2.  Call its `try_into_reader_with_file_path()` method to try to read in a CSV file matching the default read options from the specified path.
    -   Note, that you must wrap the path in a `Some()` variant and call `.into()` on the string literal itself to convert it from a string literal to a `PathBuf` (see the documentation of the method).
3.  If the reading is successful, you can call `finish()` on the resulting `Some()` variant and unwrap the final product.

Note that both reading in CSVs and finishing the reading are fallible - hence the `unwrap()` calls in the example code.

> [!WARNING]
> In real code, you should always prefer to handle the `Some(data)` and `None` cases, e.g., through pattern matching, rather than unwrapping. Unwrapping a `None` will make your program panic and crash, even if a `None` is okay to receive at a particular code point!

The `assert_eq!()` is there to show you that the same result could have been accomplished by calling the `df!` macro, just to show that the two produce similar output but for different use cases.
