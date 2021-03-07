use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::{Buf, Bytes};
use derivative::Derivative;

use crate::core::tx::Tx;
use crate::Result;

use super::fetcher::TX_FETCHER;
use super::script::Script;

#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct Input {
    #[derivative(Debug(format_with = "crate::format::bytes::fmt"))]
    pub(crate) prev_tx: Bytes, // size: 32 bytes
    pub(crate) prev_idx: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) script_sig: Script, // size: variable
    #[derivative(Debug = "ignore")]
    pub(crate) sequence: u32,
}

impl Input {
    const DEFAULT_SEQUENCE: u32 = 0xffffffff;

    pub fn new<B>(prev_tx: B, prev_idx: u32) -> Result<Self>
    where
        B: AsRef<[u8]>,
    {
        let prev_tx = Bytes::copy_from_slice(prev_tx.as_ref());
        let script_sig = Script::default();
        let sequence = Self::DEFAULT_SEQUENCE;

        Ok(Self {
            prev_tx,
            prev_idx,
            script_sig,
            sequence,
        })
    }

    pub async fn fetch_tx(&self, testnet: bool) -> Result<Tx> {
        let tx_id = hex::encode(&self.prev_tx);
        TX_FETCHER.fetch(&tx_id, testnet, false).await
    }

    pub fn value(&self, tx: &Tx) -> u64 {
        tx.outputs[self.prev_idx as usize].amount
    }

    pub fn script_pubkey<'a>(&self, tx: &'a Tx) -> &'a Script {
        &tx.outputs[self.prev_idx as usize].script_pubkey
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let prev_tx_bytes = self.prev_tx.iter().copied().rev();
        let prev_idx_bytes = self.prev_idx.to_le_bytes();
        let script_sig_bytes = self.script_sig.serialize()?.into_iter();
        let sequence_bytes = self.sequence.to_le_bytes();

        let result = prev_tx_bytes
            .chain(prev_idx_bytes.iter().copied())
            .chain(script_sig_bytes)
            .chain(sequence_bytes.iter().copied())
            .collect();

        Ok(result)
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
