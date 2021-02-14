use num_bigint::BigUint;

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
}
