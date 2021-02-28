use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::{Buf, Bytes};
use derivative::Derivative;

use crate::Result;

use super::script::Script;

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Input {
    #[derivative(Debug(format_with = "crate::format::bytes::fmt"))]
    prev_tx: Bytes, // size: 32 bytes
    prev_idx: u32,
    #[derivative(Debug = "ignore")]
    script_sig: Script, // size: variable
    #[derivative(Debug = "ignore")]
    sequence: u32,
}

impl Input {
    const DEFAULT_SEQUENCE: u32 = 0xffffffff;

    pub fn new<B>(prev_tx: B, prev_idx: u32) -> Result<Self>
    where
        B: AsRef<[u8]>,
    {
        let prev_tx = Bytes::copy_from_slice(prev_tx.as_ref());
        let script_sig = Script::new();
        let sequence = Self::DEFAULT_SEQUENCE;

        Ok(Self {
            prev_tx,
            prev_idx,
            script_sig,
            sequence,
        })
    }

    pub fn deserialize(buf: impl Buf) -> Result<Self> {
        let mut reader = buf.reader();

        let mut prev_tx_bytes = [0u8; 32];
        reader.read_exact(&mut prev_tx_bytes)?;
        prev_tx_bytes.reverse();
        let prev_tx = Bytes::copy_from_slice(&prev_tx_bytes[..]);

        let prev_idx = reader.read_u32::<LittleEndian>()?;
        let script_sig = Script::deserialize(reader.get_mut())?;
        let sequence = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            prev_tx,
            prev_idx,
            script_sig,
            sequence,
        })
    }
}
