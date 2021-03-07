use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;

use crate::Result;

use super::script::Script;

#[derive(Debug, Clone)]
pub struct Output {
    pub(crate) amount: u64,
    pub(crate) script_pubkey: Script,
}

impl Output {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let result = self
            .amount
            .to_le_bytes()
            .iter()
            .copied()
            .chain(self.script_pubkey.serialize()?)
            .collect();

        Ok(result)
    }

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
