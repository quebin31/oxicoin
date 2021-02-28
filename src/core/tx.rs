use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;

use crate::varint::VarInt;
use crate::Result;

use super::input::Input;
use super::output::Output;

#[derive(Debug, Clone)]
pub struct Tx {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    locktime: u64,
    testnet: bool,
}

impl Tx {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        todo!()
    }

    pub fn deserialize(buf: impl Buf, testnet: bool) -> Result<Self> {
        let mut reader = buf.reader();

        let version = reader.read_u32::<LittleEndian>()?;

        let no_inputs = VarInt::decode(reader.get_mut())?;
        let inputs: Vec<_> = (0..no_inputs.as_u64())
            .map(|_| Input::deserialize(reader.get_mut()))
            .collect::<Result<_, _>>()?;

        let no_outputs = VarInt::decode(reader.get_mut())?;
        let outputs: Vec<_> = (0..no_outputs.as_u64())
            .map(|_| Output::deserialize(reader.get_mut()))
            .collect::<Result<_, _>>()?;

        let locktime = reader.read_u64::<LittleEndian>()?;

        Ok(Self {
            version,
            inputs,
            outputs,
            locktime,
            testnet,
        })
    }
}
