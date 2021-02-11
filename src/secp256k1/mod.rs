use lazy_static::lazy_static;

pub mod curve;
pub mod field;
pub mod signature;

use curve::Point;
use field::FieldElement;
use num_bigint::BigUint;

lazy_static! {
    pub(crate) static ref G: Point = ec_point!(
        "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
    );
    pub(crate) static ref N: BigUint =
        biguint!("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_n() {
        let res = &*G * N.clone();
        assert!(res.is_point_at_inf());
    }
}
