/// Fast modular exponentiation
pub fn pow_mod(mut num: usize, mut exp: usize, modulus: usize) -> usize {
    let mut result = 1;

    while exp != 0 {
        if exp & 1 == 1 {
            result = (result * num) % modulus;
        }

        exp >>= 1;
        num = (num * num) % modulus;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exp_mod() {
        let r = pow_mod(7, 5, 17);
        assert_eq!(r, 11);
    }
}
