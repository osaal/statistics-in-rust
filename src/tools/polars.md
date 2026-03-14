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

> [!TIP]
> The `assert_eq!()` is there to show you that the same result could have been accomplished by calling the `df!` macro, just to show that the two produce similar output but for different use cases.

It is best to use the `CsvReadOptions::Default()` method every time, since there are a **lot** of options to set - 21 options to be exact!

However, if you need to change CSV options, you may do so either by using the associated `with_X()` methods (recommended), or by modifying the struct fields directly (less recommended).

#### CSV read options {.advanced}

This section is not necessary reading, but can help you figure out whether your CSV files are being read correctly.

The full field listing of `CsvReadOptions` (i.e., the options themselves), is available in the [documentation](https://docs.rs/polars-io/0.53.0/polars_io/csv/read/struct.CsvReadOptions.html). Some important settings are:

-   `path`: Store the file path directly in the options object, in case you need to call it multiple times or want a shorter method chain.
-   `n_rows`: How many rows should be read in?
-   `columns`: Which columns should be read in?
-   `projection`: Which columns, based on their zero-started index, should be read in?
-   `schema`: Which Polars-internal data types should be used for each column? (Default behaviour is to infer this from the file)
-   `parse_options?`: Which CSV-specific parsing options should be used?

The last one deserves its own treatment here. `CsvReadOptions.parse_options` takes an `Arc<CsvParseOptions>`. It can also be created without using a smart pointer through the `CsvReadOption::with_parse_options(CsvParseOptions)` method.

Parse options are themselves numerous, but important:

| Field name | Default value | Description |
|--|--|--|
| `separator` | `b','` | Which separator is used between the cells? |
| `quote_char` | `Some(b'"')` | Which character is used to quote strings? |
| `eol_char` | `b'\n'` | Which character marks the end of a line? |
| `encoding` | `CsvEncoding::default()` | Which text encoding is used? |
| `null_values` | `None` | Which values will be interpreted as missing, if any? |
| `missing_is_null` | `true` | Should missing fields be treated as null, following the treatment of `null_values`? |
| `truncate_ragged_lines` | `false` | Should lines with more columns than the `Schema` be truncated? |
| `comment_prefix` | `None` | What is the line prefix for a comment line, if any? |
| `try_parse_dates` | `false` | Should dates be attempted to be parsed? If failed, they become strings. |
| `decimal_comma` | `false` | Should a comma character denote the decimal separator?` |



