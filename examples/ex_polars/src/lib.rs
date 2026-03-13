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

#[inline(always)]
#[allow(dead_code)]
fn read_csv() -> Result<(), TestError> {
    // ANCHOR: read_csv
    use polars::prelude::*;
    let df = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some("src/res/myfile.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    assert_eq!(
        df,
        df!(
            "Var1" => [1,3,6,3,9],
            "Var2" => [1,7,7,4,1],
            "Var3" => [2,2,8,8,0],
        )
        .unwrap()
    );
    // ANCHOR_END: read_csv
    Ok(())
}
