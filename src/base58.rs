use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};

use crate::utils::hash256;

const BASE58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode<B>(bytes: B) -> String
where
    B: AsRef<[u8]>,
{
    lazy_static! {
        static ref BASE: BigUint = BigUint::from(58usize);
    }

    let bytes = bytes.as_ref();
    let zeroes_count = bytes.iter().take_while(|b| **b == 0).count();
    let prefix = String::from_utf8(vec![b'1'; zeroes_count]).unwrap();
    let mut number = BigUint::from_bytes_be(bytes);

    let mut result = String::new();
    while !number.is_zero() {
        let (q, r) = number.div_mod_floor(&*BASE);
        number = q;
        result.push(BASE58_ALPHABET[r.to_usize().unwrap()] as char);
    }

    result.push_str(&prefix);
    result.chars().rev().collect()
}

pub fn encode_checksum<B>(bytes: B) -> String
where
    B: AsRef<[u8]>,
{
    let checksum = hash256(bytes.as_ref());
    let data: Vec<_> = bytes
        .as_ref()
        .iter()
        .chain(&checksum[..4])
        .copied()
        .collect();
    encode(&data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn encode_base58() {
        let input = hex!("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d");
        let expected = "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6";
        assert_eq!(encode(input), expected.to_string());

        let input = hex!("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c");
        let expected = "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd";
        assert_eq!(encode(input), expected.to_string());

        let input = hex!("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6");
        let expected = "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7";
        assert_eq!(encode(input), expected.to_string());
    }
}
