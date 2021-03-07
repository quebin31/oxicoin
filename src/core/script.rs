use bytes::{Buf, Bytes};

use crate::utils::{hash160, hash256};
use crate::Result;

#[derive(Debug, Clone)]
pub enum ScriptCommand {
    OpDup,
    OpHash256,
    OpHash160,
    Element(Bytes),
}

impl ScriptCommand {
    pub fn op_from_byte(byte: u8) -> Self {
        match byte {
            0x76 => Self::OpDup,
            0xaa => Self::OpHash256,
            0xa9 => Self::OpHash160,
            invalid => unreachable!("invalid op code: {}", invalid),
        }
    }

    pub fn element_from_bytes(bytes: impl Into<Bytes>) -> Self {
        let bytes = bytes.into();
        Self::Element(bytes)
    }
}

#[derive(Debug, Clone)]
pub struct Script {
    cmds: Vec<ScriptCommand>,
}

impl Script {
    pub fn new(cmds: impl Into<Option<Vec<ScriptCommand>>>) -> Self {
        let cmds = if let Some(cmds) = cmds.into() {
            cmds
        } else {
            Vec::new()
        };

        Self { cmds }
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        todo!()
    }

    pub fn deserialize(buf: impl Buf) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptVm {
    stack: Vec<Bytes>,
}

impl ScriptVm {
    pub fn op_dup(&mut self) -> bool {
        if let Some(top) = self.stack.last().cloned() {
            self.stack.push(top);
            true
        } else {
            false
        }
    }

    pub fn op_hash256(&mut self) -> bool {
        if let Some(top) = self.stack.last() {
            let digest = hash256(top);
            self.stack.push(digest.into());
            true
        } else {
            false
        }
    }

    pub fn op_hash160(&mut self) -> bool {
        if let Some(top) = self.stack.last() {
            let digest = hash160(top);
            self.stack.push(digest.into());
            true
        } else {
            false
        }
    }
}
