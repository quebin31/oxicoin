use std::convert::TryFrom;

use byteorder::{LittleEndian, ReadBytesExt};
use bytes::Buf;

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

const U8_LIMIT: u8 = 253;
const U16_LIMIT: u16 = 65535; // 2 ^ 16 - 1
const U32_LIMIT: u32 = 4294967295; // 2 ^ 32 - 1
const U64_LIMIT: u64 = 18446744073709551615; // 2 ^ 64 - 1

impl From<u8> for VarInt {
    fn from(val: u8) -> Self {
        if val < U8_LIMIT {
            Self::U8(val)
        } else {
            Self::U16(val as u16)
        }
    }
}

impl From<u16> for VarInt {
    fn from(val: u16) -> Self {
        match val {
            val if val < U8_LIMIT as u16 => Self::U8(val as u8),
            val if val < U16_LIMIT => Self::U16(val),
            _ => Self::U32(val as u32),
        }
    }
}

impl From<u32> for VarInt {
    fn from(val: u32) -> Self {
        match val {
            val if val < U8_LIMIT as u32 => Self::U8(val as u8),
            val if val < U16_LIMIT as u32 => Self::U16(val as u16),
            val if val < U32_LIMIT => Self::U32(val),
            _ => Self::U64(val as u64),
        }
    }
}

impl TryFrom<u64> for VarInt {
    type Error = Error;

    fn try_from(val: u64) -> Result<Self, Self::Error> {
        match val {
            val if val < U8_LIMIT as u64 => Ok(Self::U8(val as u8)),
            val if val < U16_LIMIT as u64 => Ok(Self::U16(val as u16)),
            val if val < U32_LIMIT as u64 => Ok(Self::U32(val as u32)),
            val if val < U64_LIMIT => Ok(Self::U64(val)),
            _ => Err(Error::IntToBigForVarInt),
        }
    }
}

#[cfg(target_pointer_width = "16")]
impl From<usize> for VarInt {
    fn from(val: usize) -> Self {
        From::<u16>::from(val as u16)
    }
}

#[cfg(target_pointer_width = "32")]
impl From<usize> for VarInt {
    fn from(val: usize) -> Self {
        From::<u32>::from(val as u32)
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for VarInt {
    type Error = Error;

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        TryFrom::<u64>::try_from(val as u64)
    }
}

impl VarInt {
    pub fn encode(self) -> Vec<u8> {
        match self {
            VarInt::U8(val) => vec![val],
            VarInt::U16(val) => std::iter::once(0xfdu8)
                .chain(val.to_le_bytes().iter().copied())
                .collect(),

            VarInt::U32(val) => std::iter::once(0xfeu8)
                .chain(val.to_le_bytes().iter().copied())
                .collect(),

            VarInt::U64(val) => std::iter::once(0xffu8)
                .chain(val.to_le_bytes().iter().copied())
                .collect(),
        }
    }

    pub fn decode(bytes: impl Buf) -> Result<Self> {
        let mut reader = bytes.reader();

        match reader.read_u8()? {
            first if first == 0xfd => {
                let value = reader.read_u16::<LittleEndian>()?;
                Ok(Self::U16(value))
            }

            first if first == 0xfe => {
                let value = reader.read_u32::<LittleEndian>()?;
                Ok(Self::U32(value))
            }

            first if first == 0xff => {
                let value = reader.read_u64::<LittleEndian>()?;
                Ok(Self::U64(value))
            }

            value => Ok(Self::U8(value)),
        }
    }

    pub fn as_u64(self) -> u64 {
        match self {
            VarInt::U8(val) => val as u64,
            VarInt::U16(val) => val as u64,
            VarInt::U32(val) => val as u64,
            VarInt::U64(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn u8_varint() -> Result<()> {
        let varint = VarInt::from(234u8);
        assert_eq!(varint, VarInt::U8(234));

        let encoded = varint.encode();
        assert_eq!(encoded, vec![234]);

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        Ok(())
    }

    #[test]
    fn u16_varint() -> Result<()> {
        let varint = VarInt::from(U8_LIMIT + 2);
        assert_eq!(varint, VarInt::U16(255));

        let encoded = varint.encode();
        assert_eq!(encoded, vec![0xfd, 0xff, 0x00]);

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        let varint = VarInt::from(0x8549u16);
        assert_eq!(varint, VarInt::U16(0x8549));

        let encoded = varint.encode();
        assert_eq!(encoded, vec![0xfd, 0x49, 0x85]);

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        Ok(())
    }

    #[test]
    fn u32_varint() -> Result<()> {
        let varint = VarInt::from(U16_LIMIT);
        assert_eq!(varint, VarInt::U32(U16_LIMIT as u32));

        let encoded = varint.encode();
        assert_eq!(encoded, vec![0xfe, 0xff, 0xff, 0x00, 0x00]);

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        let varint = VarInt::from(0xffffd805u32);
        assert_eq!(varint, VarInt::U32(0xffffd805u32));

        let encoded = varint.encode();
        assert_eq!(encoded, vec![0xfe, 0x05, 0xd8, 0xff, 0xff]);

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        Ok(())
    }

    #[test]
    fn u64_varint() -> Result<()> {
        let varint = VarInt::from(U32_LIMIT);
        assert_eq!(varint, VarInt::U64(U32_LIMIT as u64));

        let encoded = varint.encode();
        assert_eq!(
            encoded,
            vec![0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]
        );

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        let varint = VarInt::try_from(0xffffffffffdc468du64)?;
        assert_eq!(varint, VarInt::U64(0xffffffffffdc468d));

        let encoded = varint.encode();
        assert_eq!(
            encoded,
            vec![0xff, 0x8d, 0x46, 0xdc, 0xff, 0xff, 0xff, 0xff, 0xff]
        );

        let decoded = VarInt::decode(encoded.as_slice())?;
        assert_eq!(decoded, varint);

        assert!(VarInt::try_from(U64_LIMIT).is_err());

        Ok(())
    }
}
