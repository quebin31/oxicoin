use num_bigint::BigUint;

use super::curve::Point;
use super::{GENERATOR, ORDER};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
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

    pub fn is_valid(&self, digest: &[u8; 32], pub_key: &Point) -> bool {
        let z = BigUint::from_bytes_be(digest);
        let s_inv = self.s.modpow(&(&*ORDER - 2usize), &*ORDER);

        let u = (&z * &s_inv) % &*ORDER;
        let v = (&self.r * &s_inv) % &*ORDER;

        let total = &*GENERATOR * u + pub_key * v;
        total.x().unwrap().0 == self.r
    }
}

#[cfg(test)]
mod tests {
    use crate::secp256k1::curve::Point;
    use crate::secp256k1::field::FieldElement;

    use super::Signature;

    use num_bigint::BigUint;

    #[test]
    fn valid_signature() {
        let digest = [
            0xBC, 0x62, 0xD4, 0xB8, 0x0D, 0x9E, 0x36, 0xDA, 0x29, 0xC1, 0x6C, 0x5D, 0x4D, 0x9F,
            0x11, 0x73, 0x1F, 0x36, 0x05, 0x2C, 0x72, 0x40, 0x1A, 0x76, 0xC2, 0x3C, 0x0F, 0xB5,
            0xA9, 0xB7, 0x44, 0x23,
        ];

        let r = BigUint::parse_bytes(
            b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();
        let s = BigUint::parse_bytes(
            b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();
        let signature = Signature::new(r, s);

        let x = BigUint::parse_bytes(
            b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
            16,
        )
        .unwrap();
        let y = BigUint::parse_bytes(
            b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
            16,
        )
        .unwrap();
        let pub_key = Point::new(FieldElement::new(x), FieldElement::new(y)).unwrap();

        assert!(signature.is_valid(&digest, &pub_key))
    }
}
