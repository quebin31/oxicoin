pub mod curve;
pub mod field;
pub mod utils;

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
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

    #[error("Point ({0}, {1}) is not on the curve")]
    PointNotInTheCurve(isize, isize),

    #[error("Received an invalid ec point")]
    InvalidECPoint,

    #[error("Points are not in the same curve")]
    PointsNotInTheSameCurve,
}

pub type Result<T> = std::result::Result<T, Error>;
