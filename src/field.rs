use num_bigint::{BigInt, BigUint, Sign, ToBigUint};
use num_integer::Integer;
use num_traits::{One, Pow, Zero};

use crate::traits::fragile::{FragileAdd, FragileDiv, FragileMul, FragileSub};
use crate::traits::zero::IsZero;
use crate::{forward_fragile_impl, Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement {
    prime: BigUint,
    number: BigUint,
}

impl FieldElement {
    /// Build a new field element `number` on F_{prime}.
    pub fn new<U>(number: U, prime: U) -> Result<Self, Error>
    where
        U: Into<BigUint>,
    {
        let number = number.into();
        let prime = prime.into();

        if number >= prime {
            Err(Error::InvalidFieldNumber)
        } else {
            Ok(Self { number, prime })
        }
    }

    /// Get the _additive inverse_ of this element.
    #[inline]
    pub fn add_inv(&self) -> Self {
        let number = &self.prime - &self.number;

        Self {
            number,
            prime: self.prime.clone(),
        }
    }

    /// Get the _multiplicative inverse_ of this element.
    #[inline]
    pub fn mul_inv(&self) -> Self {
        // Fermat's little theorem
        self.pow(&self.prime - &2.to_biguint().unwrap())
    }
}

impl IsZero for FieldElement {
    fn is_zero(&self) -> bool {
        <BigUint as Zero>::is_zero(&self.number)
    }
}

impl<'a, E> Pow<E> for &'a FieldElement
where
    E: Into<BigInt>,
{
    type Output = FieldElement;

    fn pow(self, exp: E) -> Self::Output {
        let exp: BigInt = exp.into();
        let exponent = match exp.to_biguint() {
            Some(exp) => exp,
            None => {
                let prime = BigInt::from_biguint(Sign::Plus, &self.prime - BigUint::one());
                exp.mod_floor(&prime).to_biguint().unwrap() // safe
            }
        };

        let number = self.number.modpow(&exponent, &self.prime);

        FieldElement {
            number,
            prime: self.prime.clone(),
        }
    }
}

impl<E> Pow<E> for FieldElement
where
    E: Into<BigInt>,
{
    type Output = FieldElement;

    fn pow(self, exp: E) -> Self::Output {
        Pow::pow(&self, exp)
    }
}

impl<'a, 'b> FragileAdd<&'a FieldElement> for &'b FieldElement {
    type Ok = FieldElement;
    type Error = Error;

    #[inline]
    fn fragile_add(self, rhs: &'a FieldElement) -> Result<Self::Ok, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldAddition)
        } else {
            let number = (&self.number + &rhs.number) % &self.prime;
            let prime = self.prime.clone();

            Ok(FieldElement { number, prime })
        }
    }
}

impl<'a, 'b> FragileSub<&'a FieldElement> for &'b FieldElement {
    type Ok = FieldElement;
    type Error = Error;

    fn fragile_sub(self, rhs: &'a FieldElement) -> Result<Self::Ok, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldSubstraction)
        } else {
            FragileAdd::fragile_add(self, &rhs.add_inv())
        }
    }
}

impl<'a, 'b> FragileMul<&'a FieldElement> for &'b FieldElement {
    type Ok = FieldElement;
    type Error = Error;

    #[inline]
    fn fragile_mul(self, rhs: &'a FieldElement) -> Result<Self::Ok, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldMultiplication)
        } else {
            // quebin31: what happens if we overflow?
            let number = (&self.number * &rhs.number) % &self.prime;
            let prime = self.prime.clone();

            Ok(FieldElement { number, prime })
        }
    }
}

impl<'a, U> FragileMul<U> for &'a FieldElement
where
    U: Into<BigUint>,
{
    type Ok = FieldElement;
    type Error = Error;

    #[inline]
    fn fragile_mul(self, rhs: U) -> Result<Self::Ok, Self::Error> {
        let scale = rhs.into();
        let scale = FieldElement::new(scale, self.prime.clone())?;
        scale.fragile_mul(self)
    }
}

impl<U> FragileMul<U> for FieldElement
where
    U: Into<BigUint>,
{
    type Ok = Self;
    type Error = Error;

    #[inline]
    fn fragile_mul(self, rhs: U) -> Result<Self::Ok, Self::Error> {
        FragileMul::fragile_mul(&self, rhs)
    }
}

impl<'a, 'b> FragileDiv<&'a FieldElement> for &'b FieldElement {
    type Ok = FieldElement;
    type Error = Error;

    #[inline]
    fn fragile_div(self, rhs: &'a FieldElement) -> Result<Self::Ok, Self::Error> {
        if self.prime != rhs.prime {
            Err(Error::InvalidFieldDivition)
        } else {
            FragileMul::fragile_mul(self, &rhs.mul_inv())
        }
    }
}

forward_fragile_impl!(for non-copyable FieldElement => FragileAdd, fragile_add, Error);
forward_fragile_impl!(for non-copyable FieldElement => FragileSub, fragile_sub, Error);
forward_fragile_impl!(for non-copyable FieldElement => FragileMul, fragile_mul, Error);
forward_fragile_impl!(for non-copyable FieldElement => FragileDiv, fragile_div, Error);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn equality() -> Result<()> {
        let a = FieldElement::new(7usize, 13)?;
        let b = FieldElement::new(6usize, 13)?;

        assert_ne!(a, b);
        Ok(())
    }

    #[test]
    fn addition() -> Result<()> {
        let a = FieldElement::new(6usize, 13)?;
        let b = FieldElement::new(7usize, 13)?;
        let c = a.fragile_add(b)?;

        assert_eq!(c, FieldElement::new(0usize, 13)?);
        Ok(())
    }

    #[test]
    fn substraction() -> Result<()> {
        let a = FieldElement::new(9usize, 57)?;
        let b = FieldElement::new(29usize, 57)?;
        let c = a.fragile_sub(b)?;

        assert_eq!(c, FieldElement::new(37usize, 57)?);
        Ok(())
    }

    #[test]
    fn multiplication() -> Result<()> {
        let a = FieldElement::new(3usize, 13)?;
        let b = FieldElement::new(12usize, 13)?;
        let c = a.fragile_mul(b)?;

        assert_eq!(c, FieldElement::new(10usize, 13)?);
        Ok(())
    }

    #[test]
    fn exponentiation() -> Result<()> {
        use num_traits::Pow;

        let a = FieldElement::new(7usize, 13)?;
        let b = (&a).pow(3usize);
        let c = (&a).pow(-3isize);

        assert_eq!(b, FieldElement::new(5usize, 13)?);
        assert_eq!(c, FieldElement::new(8usize, 13)?);
        Ok(())
    }

    #[test]
    fn divition() -> Result<()> {
        let a = FieldElement::new(2usize, 19)?;
        let b = FieldElement::new(7usize, 19)?;
        let c = a.fragile_div(b)?;

        assert_eq!(c, FieldElement::new(3usize, 19)?);
        Ok(())
    }
}
