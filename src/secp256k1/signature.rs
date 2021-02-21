use std::io::Read;

use bytes::Buf;
use num_bigint::BigUint;

use crate::utils::strip_start;
use crate::Error;

use super::crypto::PublicKey;
use super::{G, N};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub(crate) r: BigUint,
    pub(crate) s: BigUint,
}

impl Signature {
    pub fn new<U>(r: U, s: U) -> Self
    where
        U: Into<BigUint>,
    {
        let r = r.into();
        let s = s.into();
        Self { r, s }
    }

    pub fn is_valid<B>(&self, digest: B, pub_key: &PublicKey) -> Result<bool, Error>
    where
        B: AsRef<[u8]>,
    {
        let digest = digest.as_ref();
        if digest.len() != 32 {
            return Err(Error::InvalidDigestLength(digest.len()));
        }

        let z = BigUint::from_bytes_be(digest);
        let s_inv = self.s.modpow(&(&*N - 2usize), &*N);

        let u = (&z * &s_inv) % &*N;
        let v = (&self.r * &s_inv) % &*N;

        let total = &*G * u + &pub_key.ec_point * v;
        Ok(total.x().unwrap().0 == self.r)
    }

    /// Serialize signature with DER format
    pub fn serialize(&self) -> Result<Vec<u8>, Error> {
        let r_bigendian = self.r.to_bytes_be();
        let r_bigendian = strip_start(&r_bigendian, 0x00);
        let r_bigendian = if r_bigendian[0] & 0x80 == 0x80 {
            std::iter::once(0x00u8)
                .chain(r_bigendian.iter().copied())
                .collect::<Vec<_>>()
        } else {
            r_bigendian.to_vec()
        };

        let prefix = [0x02u8, r_bigendian.len() as u8];
        let result = prefix.iter().copied().chain(r_bigendian);

        let s_bigendian = self.s.to_bytes_be();
        let s_bigendian = strip_start(&s_bigendian, 0x00);
        let s_bigendian = if s_bigendian[0] & 0x80 == 0x80 {
            std::iter::once(0x00u8)
                .chain(s_bigendian.iter().copied())
                .collect::<Vec<_>>()
        } else {
            s_bigendian.to_vec()
        };

        let result = result
            .chain([0x02u8, s_bigendian.len() as u8].iter().copied())
            .chain(s_bigendian)
            .collect::<Vec<_>>();

        let serialized = [0x30u8, result.len() as u8]
            .iter()
            .copied()
            .chain(result)
            .collect();

        Ok(serialized)
    }

    pub fn deserialize<B: Buf>(bytes: B) -> Result<Self, Error> {
        let size = bytes.remaining();
        let mut reader = bytes.reader();

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;

        if buf[0] != 0x30 {
            return Err(Error::InvalidSignature("bad compound"));
        }

        let claimed_size = (buf[1] + 2) as usize;
        if claimed_size != size {
            return Err(Error::InvalidSignature("bad signature size"));
        }

        if buf[2] != 0x02 {
            return Err(Error::InvalidSignature("bad marker"));
        }

        let r_size = buf[3] as usize;
        let mut r_bytes = vec![0u8; r_size];
        reader.read_exact(&mut r_bytes)?;
        let r = BigUint::from_bytes_be(&r_bytes);

        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;

        if buf[0] != 0x02 {
            return Err(Error::InvalidSignature("bad marker"));
        }

        let s_size = buf[0] as usize;
        let mut s_bytes = vec![0u8; s_size];
        reader.read_exact(&mut s_bytes)?;
        let s = BigUint::from_bytes_be(&s_bytes);

        if size != 6 + r_size + s_size {
            return Err(Error::InvalidSignature("signature too long"));
        }

        Ok(Self { r, s })
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::Signature;

    #[test]
    fn der_format() {
        let r = biguint!("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = biguint!("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        let signature = Signature::new(r, s);

        let serialized = signature.serialize().unwrap();
        let expected = hex!(
            "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6022100
            8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
        );

        assert_eq!(serialized, expected);
    }
}
