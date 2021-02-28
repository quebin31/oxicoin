use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;

use crate::Result;

use super::script::Script;

#[derive(Debug, Clone)]
pub struct Output {
    amount: u64,
    script_pubkey: Script,
}

impl Output {
    pub fn deserialize(buf: impl Buf) -> Result<Self> {
        let mut reader = buf.reader();

        let amount = reader.read_u64::<LittleEndian>()?;
        let script_pubkey = Script::deserialize(reader.get_mut())?;

        Ok(Self {
            amount,
            script_pubkey,
        })
    }
}
