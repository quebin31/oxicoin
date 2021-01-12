use crate::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElem {
    number: usize,
    prime: usize,
}

impl FieldElem {
    pub fn new(number: usize, prime: usize) -> Result<Self, Error> {
        if number >= prime {
            Err(Error::InvalidFieldNumber(number, prime))
        } else {
            Ok(Self { number, prime })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn equality() -> Result<()> {
        let a = FieldElem::new(7, 13)?;
        let b = FieldElem::new(6, 13)?;

        assert_ne!(a, b);
        assert_eq!(b, b);

        Ok(())
    }
}
