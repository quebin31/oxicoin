use std::ops::{Add, Div, Mul, Sub};

use lazy_static::lazy_static;
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Pow, Zero};

lazy_static! {
    /// `secp256k1` prime = 2^256 - 2^32 - 977
    pub(crate) static ref PRIME: BigUint =
        biguint!("fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldElement(pub(crate) BigUint);

impl FieldElement {
    /// Build a new element in the S256 field
    pub fn new<U>(number: U) -> Self
    where
        U: Into<BigUint>,
    {
        Self(number.into() % &*PRIME)
    }

    /// Get the _additive inverse_ of this element.
    #[inline]
    pub fn add_inv(&self) -> Self {
        Self(&*PRIME - &self.0)
    }

    /// Get the _multiplicative inverse_ of this element.
    #[inline]
    pub fn mul_inv(&self) -> Self {
        // Fermat's little theorem
        self.pow(&*PRIME - 2usize)
    }
}

impl Zero for FieldElement {
    fn zero() -> Self {
        FieldElement(BigUint::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl One for FieldElement {
    fn one() -> Self {
        FieldElement(BigUint::one())
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
                let prime = BigInt::from_biguint(Sign::Plus, &*PRIME - BigUint::one());
                exp.mod_floor(&prime).to_biguint().unwrap() // safe
            }
        };

        let number = self.0.modpow(&exponent, &*PRIME);
        FieldElement(number)
    }
}

impl<'a, 'b> Add<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: &'a FieldElement) -> Self::Output {
        let number = (&self.0 + &rhs.0) % &*PRIME;
        FieldElement::new(number)
    }
}

impl<'a, 'b> Sub<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: &'a FieldElement) -> Self::Output {
        self.add(&rhs.add_inv())
    }
}

impl<'a, 'b> Mul<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        let number = (&self.0 * &rhs.0) % &*PRIME;
        FieldElement::new(number)
    }
}

impl<'a, 'b> Div<&'a FieldElement> for &'b FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: &'a FieldElement) -> Self::Output {
        self.mul(&rhs.mul_inv())
    }
}

impl<'a> Mul<usize> for &'a FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: usize) -> Self::Output {
        self.mul(FieldElement::new(rhs))
    }
}

impl Mul<usize> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: usize) -> Self::Output {
        Mul::mul(&self, rhs)
    }
}

forward_binop_impl!(for non-copyable FieldElement where Add does add);
forward_binop_impl!(for non-copyable FieldElement where Sub does sub);
forward_binop_impl!(for non-copyable FieldElement where Mul does mul);
forward_binop_impl!(for non-copyable FieldElement where Div does div);
