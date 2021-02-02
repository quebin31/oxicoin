pub mod curve;
pub mod field;
pub mod traits;

use std::convert::Infallible;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("custom error")]
    Custom(String),

    #[error("invalid field number, not in range")]
    InvalidFieldNumber,

    #[error("invalid addition on different fields")]
    InvalidFieldAddition,

    #[error("invalid substraction on different fields")]
    InvalidFieldSubstraction,

    #[error("invalid multiplication on different fields")]
    InvalidFieldMultiplication,

    #[error("invalid divition on different fields")]
    InvalidFieldDivition,

    #[error("point is not on the curve")]
    PointNotInTheCurve,

    #[error("received an invalid ec point")]
    InvalidECPoint,

    #[error("points are not in the same curve")]
    PointsNotInTheSameCurve,
}

impl Error {
    pub fn custom<T: ToString>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!("how do you dare?")
    }
}

pub type Result<T> = std::result::Result<T, Error>;
