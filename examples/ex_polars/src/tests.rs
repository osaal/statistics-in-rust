use crate::*;

#[test]
fn test_use_polars() {
    assert!(use_polars().is_ok())
}

#[test]
fn test_empty_df() {
    assert!(empty_df().is_ok())
}
