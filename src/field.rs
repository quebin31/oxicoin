use num_traits::Pow;

use crate::traits::{IsZero, MayAdd, MayDiv, MayMul, MaySub};
use crate::utils::pow_mod;
use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldElement {
    number: usize,
    prime: usize,
}

impl FieldElement {
    /// Build a new field element `number` on F_{prime}.
    pub fn new(number: usize, prime: usize) -> Result<Self, Error> {
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
        self.pow(self.prime - 2)
    }
}

impl IsZero for FieldElement {
    fn is_zero(&self) -> bool {
        self.number == 0
    }
}

impl Pow<usize> for FieldElement {
    type Output = Self;

    fn pow(self, exp: usize) -> Self::Output {
        let number = pow_mod(self.number, exp, self.prime);
        Self { number, ..self }
    }
}

impl Pow<isize> for FieldElement {
    type Output = Self;

    fn pow(self, exp: isize) -> Self::Output {
        // Rust's `%` operator is not the modulus operation.
        let exp = exp.rem_euclid(self.prime as isize - 1);
        self.pow(exp as usize)
    }
}

impl MayAdd for FieldElement {
    type Output = Self;
    type Error = Error;

    #[inline]
    fn may_add(self, rhs: Self) -> Result<Self::Output, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldAddition)
        } else {
            let number = (self.number + rhs.number) % self.prime;
            Ok(Self { number, ..self })
        }
    }
}

impl MaySub for FieldElement {
    type Output = Self;
    type Error = Error;

    #[inline]
    fn may_sub(self, rhs: Self) -> Result<Self::Output, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldSubstraction)
        } else {
            self.may_add(rhs.add_inv())
        }
    }
}

impl MayMul for FieldElement {
    type Output = Self;
    type Error = Error;

    #[inline]
    fn may_mul(self, rhs: Self) -> Result<Self::Output, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldMultiplication)
        } else {
            // quebin31: what happens if we overflow?
            let number = (self.number * rhs.number) % self.prime;
            Ok(Self { number, ..self })
        }
    }
}

impl MayMul<usize> for FieldElement {
    type Output = Self;
    type Error = Error;

    #[inline]
    fn may_mul(self, rhs: usize) -> Result<Self::Output, Self::Error> {
        let this = Self::new(rhs, self.prime)?;
        this.may_mul(self)
    }
}

impl MayDiv for FieldElement {
    type Output = Self;
    type Error = Error;

    #[inline]
    fn may_div(self, rhs: Self) -> Result<Self::Output, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldDivition)
        } else {
            self.may_mul(rhs.mul_inv())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn equality() -> Result<()> {
        let a = FieldElement::new(7, 13)?;
        let b = FieldElement::new(6, 13)?;

        assert_ne!(a, b);
        Ok(())
    }

    #[test]
    fn addition() -> Result<()> {
        let a = FieldElement::new(6, 13)?;
        let b = FieldElement::new(7, 13)?;
        let c = a.may_add(b)?;

        assert_eq!(c, FieldElement::new(0, 13)?);
        Ok(())
    }

    #[test]
    fn substraction() -> Result<()> {
        let a = FieldElement::new(9, 57)?;
        let b = FieldElement::new(29, 57)?;
        let c = a.may_sub(b)?;

        assert_eq!(c, FieldElement::new(37, 57)?);
        Ok(())
    }

    #[test]
    fn multiplication() -> Result<()> {
        let a = FieldElement::new(3, 13)?;
        let b = FieldElement::new(12, 13)?;
        let c = a.may_mul(b)?;

        assert_eq!(c, FieldElement::new(10, 13)?);
        Ok(())
    }

    #[test]
    fn exponentiation() -> Result<()> {
        use num_traits::Pow;

        let a = FieldElement::new(7, 13)?;
        let b = a.pow(3usize);
        let c = a.pow(-3isize);

        assert_eq!(b, FieldElement::new(5, 13)?);
        assert_eq!(c, FieldElement::new(8, 13)?);
        Ok(())
    }

    #[test]
    fn divition() -> Result<()> {
        let a = FieldElement::new(2, 19)?;
        let b = FieldElement::new(7, 19)?;
        let c = a.may_div(b)?;

        assert_eq!(c, FieldElement::new(3, 19)?);
        Ok(())
    }
}
