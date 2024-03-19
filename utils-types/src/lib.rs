use std::fmt::Display;
use clap::ArgMatches;
use thiserror::Error;
pub mod errors;
/// Error type for forwarding Errors out of `main()` of a `clap` app
/// and still using the `Display` formatter
#[derive(Error, Debug)]
#[error("{0}")]
pub struct DisplayError(Box<dyn std::error::Error>);
impl DisplayError {
    pub fn new_as_boxed(inner: Box<dyn std::error::Error>) -> Box<Self> {
        DisplayError(inner).into()
    }
}

