mod macros;
pub mod secp256k1;
mod utils;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("custom error: {0}")]
    Custom(String),

    #[error("point is not on the curve")]
    PointNotInTheCurve,

    #[error("overflow error while padding")]
    OverflowPadding,
}

impl Error {
    pub fn custom<T: ToString>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
