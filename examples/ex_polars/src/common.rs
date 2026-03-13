use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TestError {
    UsePolars,
    EmptyDf,
}

impl Error for TestError {}

impl Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UsePolars => write!(f, "Error in use_polars()"),
            Self::EmptyDf => write!(f, "Error in empty_df()"),
        }
    }
}
