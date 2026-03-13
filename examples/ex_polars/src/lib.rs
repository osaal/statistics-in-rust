mod common;
use crate::common::TestError;

#[cfg(test)]
mod tests;

#[inline(always)]
#[allow(dead_code)]
fn use_polars() -> Result<(), TestError> {
    // ANCHOR: df_macro
    use polars::prelude::*;

    let _df = df!(
        "A" => [1, 2, 3],
        "B" => [4, 5, 6]
    );
    // ANCHOR_END: df_macro
    Ok(())
}

#[inline(always)]
#[allow(dead_code)]
fn empty_df() -> Result<(), TestError> {
    // ANCHOR: empty_df
    use polars::prelude::*;

    let _df = DataFrame::empty();
    // ANCHOR_END: empty_df
    Ok(())
}
