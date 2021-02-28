use std::convert::TryFrom;

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;

use crate::utils::hash256;
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
    pub fn id(&self) -> Result<String> {
        Ok(hex::encode(self.hash()?))
    }

    pub fn hash(&self) -> Result<Vec<u8>> {
        let serialized = self.serialize()?;
        let mut digest = hash256(&serialized);
        digest.reverse();
        Ok(digest)
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let version_bytes = self.version.to_le_bytes();

        let no_inputs = VarInt::try_from(self.inputs.len())?;
        let no_inputs_bytes = no_inputs.serialize().into_iter();

        let no_outputs = VarInt::try_from(self.outputs.len())?;
        let no_outputs_bytes = no_outputs.serialize().into_iter();

        let inputs_bytes = self
            .inputs
            .iter()
            .map(|input| input.serialize())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten();

        let outputs_bytes = self
            .outputs
            .iter()
            .map(|output| output.serialize())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten();

        let locktime_bytes = self.locktime.to_le_bytes();

        let result = version_bytes
            .iter()
            .copied()
            .chain(no_inputs_bytes)
            .chain(inputs_bytes)
            .chain(no_outputs_bytes)
            .chain(outputs_bytes)
            .chain(locktime_bytes.iter().copied())
            .collect();

        Ok(result)
    }

    pub fn deserialize(buf: impl Buf, testnet: bool) -> Result<Self> {
        let mut reader = buf.reader();

        let version = reader.read_u32::<LittleEndian>()?;

        let no_inputs = VarInt::deserialize(reader.get_mut())?;
        let inputs: Vec<_> = (0..no_inputs.as_u64())
            .map(|_| Input::deserialize(reader.get_mut()))
            .collect::<Result<_, _>>()?;

        let no_outputs = VarInt::deserialize(reader.get_mut())?;
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
