#[macro_use]
mod macros;
pub mod base58;
pub mod secp256k1;
pub mod utils;

use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("custom error: {0}")]
    Custom(String),

    #[error("io error: {source}")]
    IoError {
        #[from]
        source: io::Error,
    },

    #[error("point is not on the curve")]
    PointNotOnTheCurve,

    #[error("overflow error while padding")]
    OverflowPadding,

    #[error("cannot serialize point at infinity")]
    SerializePointAtInfinity,

    #[error("invalid digest, expecting 32 bytes, got {0}")]
    InvalidDigestLength(usize),

    #[error("invalid sec bytes, expecting either 33 or 65 bytes, got {0} ")]
    InvalidSecBytesLength(usize),

    #[error("invalid signature ({0})")]
    InvalidSignature(&'static str),
}

impl Error {
    pub fn custom<T: ToString>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
