use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
pub enum TestError {}

impl Error for TestError {}

impl Display for TestError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
