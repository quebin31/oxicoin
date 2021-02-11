use num_bigint::BigUint;

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

    pub fn is_valid(&self, digest: &[u8; 32], pub_key: &PublicKey) -> bool {
        let z = BigUint::from_bytes_be(digest);
        let s_inv = self.s.modpow(&(&*N - 2usize), &*N);

        let u = (&z * &s_inv) % &*N;
        let v = (&self.r * &s_inv) % &*N;

        let total = &*G * u + &pub_key.ec_point * v;
        total.x().unwrap().0 == self.r
    }
}

#[cfg(test)]
mod tests {
    use crate::secp256k1::crypto::PublicKey;

    use super::Signature;

    use hex_literal::hex;

    #[test]
    fn must_be_valid() {
        let digest = hex!("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423");

        let r = biguint!("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6");
        let s = biguint!("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec");
        let signature = Signature::new(r, s);

        let x = biguint!("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574");
        let y = biguint!("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4");
        let pub_key = PublicKey::new(x, y).unwrap();

        assert!(signature.is_valid(&digest, &pub_key))
    }
}
