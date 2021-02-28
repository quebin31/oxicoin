use bytes::Buf;

use crate::Result;

#[derive(Debug, Clone)]
pub struct Script {}

impl Script {
    pub fn new() -> Self {
        Self {}
    }

    pub fn deserialize(buf: impl Buf) -> Result<Self> {
        todo!()
    }
}
