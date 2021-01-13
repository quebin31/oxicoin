use std::ops::{Add, Div, Mul, Sub};

use crate::utils::pow_mod;
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElem {
    number: usize,
    prime: usize,
}

impl FieldElem {
    /// Build a new field element `number` on F_{prime}.
    pub fn new(number: usize, prime: usize) -> Result<Self> {
        if number >= prime {
            Err(Error::InvalidFieldNumber(number, prime))
        } else {
            Ok(Self { number, prime })
        }
    }

    /// Get the _additive inverse_ of this element.
    #[inline]
    pub fn add_inv(self) -> Self {
        let number = self.prime - self.number;
        Self { number, ..self }
    }

    /// Get the _multiplicative inverse_ of this element.
    #[inline]
    pub fn mul_inv(self) -> Self {
        // Fermat's little theorem
        self.powu(self.prime - 2)
    }

    #[inline]
    pub fn powu(self, exp: usize) -> Self {
        let number = pow_mod(self.number, exp, self.prime);
        Self { number, ..self }
    }

    #[inline]
    pub fn powi(self, exp: isize) -> Self {
        // Rust's `%` operator is not the modulus operation.
        let exp = exp.rem_euclid(self.prime as isize - 1);
        self.powu(exp as usize)
    }
}

impl Add for FieldElem {
    type Output = Result<Self>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldAddition)
        } else {
            let number = (self.number + rhs.number) % self.prime;
            Ok(Self { number, ..self })
        }
    }
}

impl Sub for FieldElem {
    type Output = Result<Self>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldSubstraction)
        } else {
            self.add(rhs.add_inv())
        }
    }
}

impl Mul for FieldElem {
    type Output = Result<Self>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldMultiplication)
        } else {
            // quebin31: what happens if we overflow?
            let number = (self.number * rhs.number) % self.prime;
            Ok(Self { number, ..self })
        }
    }
}

impl Div for FieldElem {
    type Output = Result<Self>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldDivition)
        } else {
            self.mul(rhs.mul_inv())
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

    #[test]
    fn addition() -> Result<()> {
        let a = FieldElem::new(6, 13)?;
        let b = FieldElem::new(7, 13)?;
        let c = a.add(b)?;

        assert_eq!(c, FieldElem::new(0, 13)?);
        Ok(())
    }

    #[test]
    fn substraction() -> Result<()> {
        let a = FieldElem::new(9, 57)?;
        let b = FieldElem::new(29, 57)?;
        let c = a.sub(b)?;

        assert_eq!(c, FieldElem::new(37, 57)?);
        Ok(())
    }

    #[test]
    fn multiplication() -> Result<()> {
        let a = FieldElem::new(3, 13)?;
        let b = FieldElem::new(12, 13)?;
        let c = a.mul(b)?;

        assert_eq!(c, FieldElem::new(10, 13)?);
        Ok(())
    }

    #[test]
    fn exponentiation() -> Result<()> {
        let a = FieldElem::new(7, 13)?;
        let b = a.powu(3);
        let c = a.powi(-3);

        assert_eq!(b, FieldElem::new(5, 13)?);
        assert_eq!(c, FieldElem::new(8, 13)?);
        Ok(())
    }

    #[test]
    fn divition() -> Result<()> {
        let a = FieldElem::new(2, 19)?;
        let b = FieldElem::new(7, 19)?;
        let c = a.div(b)?;

        assert_eq!(c, FieldElem::new(3, 19)?);
        Ok(())
    }
}
