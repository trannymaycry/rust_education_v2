pub mod cli_app;
pub mod go;
pub mod dapp;
pub mod langs;
pub mod rust;
pub mod dbg;

use crate::cli_app::ProcessResult;

pub fn execute_help(message: String) -> ProcessResult {
    Ok(message)
}
