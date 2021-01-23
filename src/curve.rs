use std::error::Error as StdError;

use num_traits::Pow;

use crate::traits::{IsZero, MayAdd, MayDiv, MayMul, MaySub};
use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EllipticCurve<T> {
    a: T,
    b: T,
}

impl<T> EllipticCurve<T> {
    pub fn new(a: T, b: T) -> Self {
        Self { a, b }
    }

    pub fn contains<E>(&self, x: T, y: T) -> Result<bool, E>
    where
        T: Copy + Eq,
        T: Pow<usize, Output = T>,
        T: MayMul<Output = T, Error = E> + MayAdd<Output = T, Error = E>,
    {
        Ok(y.pow(2) == x.pow(3).may_add(self.a.may_mul(x)?).may_add(self.b)?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point<T> {
    AtInfinity,
    Normal(T, T, EllipticCurve<T>),
}

impl<T> Point<T> {
    pub fn new<E>(x: T, y: T, curve: EllipticCurve<T>) -> Result<Self, Error>
    where
        E: StdError + Send + Sync + 'static,
        T: Copy + Eq,
        T: Pow<usize, Output = T>,
        T: MayMul<Output = T, Error = E> + MayAdd<Output = T, Error = E>,
    {
        if curve.contains(x, y).map_err(Error::from_err)? {
            Ok(Self::Normal(x, y, curve))
        } else {
            Err(Error::PointNotInTheCurve)
        }
    }

    pub fn at_infinity() -> Self {
        Self::AtInfinity
    }

    pub fn x(&self) -> Option<&T> {
        match self {
            Point::AtInfinity { .. } => None,
            Point::Normal(x, _, _) => Some(x),
        }
    }

    pub fn y(&self) -> Option<&T> {
        match self {
            Point::AtInfinity { .. } => None,
            Point::Normal(_, y, _) => Some(y),
        }
    }

    pub fn curve(&self) -> Option<&EllipticCurve<T>> {
        match self {
            Point::AtInfinity => None,
            Point::Normal(_, _, curve) => Some(curve),
        }
    }

    pub fn same_curve(&self, other: &Self) -> bool
    where
        T: Eq,
    {
        match (self.curve(), other.curve()) {
            (Some(curve1), Some(curve2)) => curve1 == curve2,
            _ => true, // One is a point at infinity
        }
    }

    pub fn is_point_at_inf(&self) -> bool {
        matches!(self, Self::AtInfinity)
    }
}

impl<T, E> MayAdd for Point<T>
where
    E: StdError + Send + Sync + 'static,
    T: Copy + Eq,
    T: IsZero,
    T: Pow<usize, Output = T>,
    T: MayAdd<Output = T, Error = E> + MaySub<Output = T, Error = E>,
    T: MayMul<Output = T, Error = E> + MayDiv<Output = T, Error = E>,
    T: MayMul<usize, Output = T, Error = E>, // scalar mul
{
    type Output = Self;
    type Error = Error;

    fn may_add(self, other: Self) -> Result<Self::Output, Self::Error> {
        if !self.same_curve(&other) {
            return Err(Error::PointsNotInTheSameCurve);
        }

        match (self, other) {
            // Additive identity
            (Self::AtInfinity, _) => Ok(other),
            (_, Self::AtInfinity) => Ok(self),

            // Normal addition between points
            (Self::Normal(x1, y1, curve), Self::Normal(x2, y2, _)) => match (x1 == x2, y1 == y2) {
                // Same x axis, rhs is additive inverse of self and viceversa
                (true, false) => Ok(Self::at_infinity()),

                // Same x and y axis, self is equal to rhs
                (true, true) => {
                    if y1.is_zero() {
                        return Ok(Self::at_infinity());
                    }

                    let slope = (x1.pow(2usize).may_mul(3).may_add(curve.a))
                        .may_div(y1.may_mul(2))
                        .map_err(Error::from_err)?;

                    let x3 = Ok(slope.pow(2usize))
                        .may_sub(x1.may_mul(2))
                        .map_err(Error::from_err)?;

                    let y3 = slope
                        .may_mul(x1.may_sub(x3))
                        .may_sub(y1)
                        .map_err(Error::from_err)?;

                    Self::new(x3, y3, curve)
                }

                // Different x axis, y axis doesn't matter in this case
                _ => {
                    let slope = y2
                        .may_sub(y1)
                        .may_div(x2.may_sub(x1))
                        .map_err(Error::from_err)?;

                    let x3 = slope
                        .may_mul(slope)
                        .may_sub(x1)
                        .may_sub(x2)
                        .map_err(Error::from_err)?;

                    let y3 = slope
                        .may_mul(x1.may_sub(x3))
                        .may_sub(y1)
                        .map_err(Error::from_err)?;

                    Self::new(x3, y3, curve)
                }
            },
        }
    }
}

impl<T, E> MayMul<usize> for Point<T>
where
    E: StdError + Send + Sync + 'static,
    T: Copy + Eq,
    T: IsZero,
    T: Pow<usize, Output = T>,
    T: MayAdd<Output = T, Error = E> + MaySub<Output = T, Error = E>,
    T: MayMul<Output = T, Error = E> + MayDiv<Output = T, Error = E>,
    T: MayMul<usize, Output = T, Error = E>, // scalar mul
{
    type Output = Self;
    type Error = Error;

    fn may_mul(self, other: usize) -> Result<Self::Output, Self::Error> {
        let mut result = self;
        for _ in 0..other - 1 {
            result = result.may_add(self)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn not_in_curve() {
        let res = Point::new(-1, -2, EllipticCurve::new(5, 7));
        assert!(res.is_err());
    }

    #[test]
    fn equality() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(18, 77, curve)?;

        assert_ne!(a, b);
        Ok(())
    }

    #[test]
    fn addition_with_inf() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let inf = Point::at_infinity();

        assert_eq!(a.may_add(inf)?, a);
        assert_eq!(inf.may_add(a)?, a);

        Ok(())
    }

    #[test]
    fn addition_with_inverse() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(-1, 1, curve)?;

        assert_eq!(a.may_add(b)?, Point::at_infinity());
        assert_eq!(b.may_add(a)?, Point::at_infinity());

        Ok(())
    }

    #[test]
    fn addition_diff_points() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(2, 5, curve)?;
        let c = Point::new(3, -7, curve)?;

        assert_eq!(a.may_add(b)?, c);
        assert_eq!(b.may_add(a)?, c);

        Ok(())
    }

    #[test]
    fn addition_equal_points() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(18, 77, curve)?;

        assert_eq!(a.may_add(a)?, b);

        Ok(())
    }

    #[test]
    fn addition_with_field_element() -> Result<()> {
        use crate::field::FieldElement;

        let prime = 223;
        let curve = EllipticCurve::new(FieldElement::new(0, prime)?, FieldElement::new(7, prime)?);

        let a = Point::new(
            FieldElement::new(192, prime)?,
            FieldElement::new(105, prime)?,
            curve,
        )?;

        let b = Point::new(
            FieldElement::new(17, prime)?,
            FieldElement::new(56, prime)?,
            curve,
        )?;

        let c = Point::new(
            FieldElement::new(170, prime)?,
            FieldElement::new(142, prime)?,
            curve,
        )?;

        assert_eq!(c, a.may_add(b)?);
        Ok(())
    }

    #[test]
    fn scalar_multiplication_with_field_element() -> Result<()> {
        use crate::field::FieldElement;

        let prime = 223;
        let curve = EllipticCurve::new(FieldElement::new(0, prime)?, FieldElement::new(7, prime)?);

        let a = Point::new(
            FieldElement::new(47, prime)?,
            FieldElement::new(71, prime)?,
            curve,
        )?;

        /*
        let b = Point::new(
            FieldElement::new(47, prime)?,
            FieldElement::new(152, prime)?,
            curve,
        )?;
        */

        println!("{:?}", a.may_mul(2)?);
        Ok(())
    }
}
