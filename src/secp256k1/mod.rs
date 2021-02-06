use lazy_static::lazy_static;

pub mod curve;
pub mod field;
pub mod signature;

use curve::Point;
use field::FieldElement;
use num_bigint::BigUint;

const GX_IN_HEX: &[u8; 64] = b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
const GY_IN_HEX: &[u8; 64] = b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8";
const ORDER_IN_HEX: &[u8; 64] = b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141";

macro_rules! field_elem {
    ($hex:expr) => {
        FieldElement::new(BigUint::parse_bytes($hex, 16).unwrap())
    };
}

lazy_static! {
    pub(crate) static ref GENERATOR: Point =
        Point::new(field_elem!(GX_IN_HEX), field_elem!(GY_IN_HEX)).unwrap();
    pub(crate) static ref ORDER: BigUint = BigUint::parse_bytes(ORDER_IN_HEX, 16).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_n() {
        let res = &*GENERATOR * ORDER.clone();
        assert_eq!(Point::at_infinity(), res);
    }
}
