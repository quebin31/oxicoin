pub mod curve;
pub mod field;
pub mod traits;
pub mod utils;

use std::error::Error as StdError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Something went wrong: {source}")]
    AnyError { source: anyhow::Error },

    #[error("Invalid field number {0}, not in range 0..{1}")]
    InvalidFieldNumber(usize, usize),

    #[error("Invalid addition on different fields")]
    InvalidFieldAddition,

    #[error("Invalid substraction on different fields")]
    InvalidFieldSubstraction,

    #[error("Invalid multiplication on different fields")]
    InvalidFieldMultiplication,

    #[error("Invalid divition on different fields")]
    InvalidFieldDivition,

    #[error("Point is not on the curve")]
    PointNotInTheCurve,

    #[error("Received an invalid ec point")]
    InvalidECPoint,

    #[error("Points are not in the same curve")]
    PointsNotInTheSameCurve,
}

impl Error {
    pub fn from_err<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self::AnyError { source: err.into() }
    }
}

#[derive(Debug, Clone, Error)]
#[error("Infallible error!")]
pub struct Infallible;

pub type Result<T> = std::result::Result<T, Error>;
