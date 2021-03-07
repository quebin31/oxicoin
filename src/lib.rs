#[macro_use]
mod macros;
pub mod base58;
pub mod core;
mod format;
pub mod secp256k1;
pub mod utils;
pub mod varint;

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

    #[error("hyper error: {source}")]
    HyperError {
        #[from]
        source: hyper::Error,
    },

    #[error("int to big for varint")]
    IntToBigForVarInt,

    #[error("invalid bytes for varint")]
    InvalidBytesForVarInt,

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

    #[error("fetched invalid transaction")]
    FetchedInvalidTransaction,
}

impl Error {
    pub fn custom<T: ToString>(msg: T) -> Self {
        Self::Custom(msg.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
