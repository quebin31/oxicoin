pub mod field;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Invalid field number {0}, not in range 0..{1}")]
    InvalidFieldNumber(usize, usize),
}

pub type Result<T> = std::result::Result<T, Error>;
