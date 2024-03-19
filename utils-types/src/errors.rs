use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Reasons the program may fail
#[derive(Clone, Debug, Deserialize, Eq, Error, PartialEq, Serialize)]
pub enum ProgramError {
    #[error("Oh error")]
    OhError,
}