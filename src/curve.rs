use num_bigint::BigUint;
use num_traits::{One, Pow};
use try_block::try_block;

use crate::traits::fragile::{FragileAdd, FragileDiv, FragileMul, FragileSub};
use crate::traits::zero::IsZero;
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

    pub fn contains<'a, E>(&'a self, x: &'a T, y: &'a T) -> Result<bool, Error>
    where
        E: Into<Error>,
        T: FragileAdd<T, Ok = T, Error = E> + Eq,
        T: FragileAdd<&'a T, Ok = T, Error = E>,
        T: FragileMul<&'a T, Ok = T, Error = E>,
        &'a T: Pow<usize, Output = T>,
        &'a T: FragileMul<Ok = T, Error = E>,
    {
        let contained: Result<_, E> = try_block! {
            let lhs = y.pow(2);
            let rhs = x
                .pow(3)
                .fragile_add((&self.a).fragile_mul(x)?)?
                .fragile_add(&self.b)?;

            Ok(lhs == rhs)
        };

        contained.map_err(Into::into)
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
        E: Into<Error>,
        T: FragileAdd<Ok = T, Error = E> + Eq,
        for<'a> T: FragileAdd<&'a T, Ok = T, Error = E>,
        for<'a> T: FragileMul<&'a T, Ok = T, Error = E>,
        for<'a> &'a T: Pow<usize, Output = T>,
        for<'a> &'a T: FragileMul<Ok = T, Error = E>,
    {
        if curve.contains(&x, &y)? {
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

impl<'a, 'b, T, E> FragileAdd<&'a Point<T>> for &'b Point<T>
where
    E: Into<Error>,
    T: IsZero + Eq + Clone,
    T: FragileAdd<Ok = T, Error = E>,
    T: FragileSub<Ok = T, Error = E>,
    T: FragileDiv<Ok = T, Error = E>,
    T: FragileMul<Ok = T, Error = E>,
    T: FragileMul<usize, Ok = T, Error = E>,
    for<'c> T: FragileAdd<&'c T, Ok = T, Error = E>,
    for<'c> T: FragileSub<&'c T, Ok = T, Error = E>,
    for<'c> &'c T: Pow<usize, Output = T>,
    for<'c> &'c T: FragileSub<&'c T, Ok = T, Error = E>,
    for<'c> &'c T: FragileMul<usize, Ok = T, Error = E>,
{
    type Ok = Point<T>;
    type Error = Error;

    fn fragile_add(self, rhs: &'a Point<T>) -> Result<Self::Ok, Self::Error> {
        if !self.same_curve(rhs) {
            return Err(Error::PointsNotInTheSameCurve);
        }

        let point: Result<_, E> = try_block! {
            match (self, rhs) {
                // Additive identity
                (Point::AtInfinity, r) => Ok(r.to_owned()),
                (l, Point::AtInfinity) => Ok(l.to_owned()),

                // Normal addition between points
                (Point::Normal(x1, y1, curve), Point::Normal(x2, y2, _)) => {
                    match (x1 == x2, y1 == y2) {
                        // Same x axis, rhs is additive inverse of self and viceversa
                        (true, false) => Ok(Point::at_infinity()),

                        // Same x and y axis, self is equal to rhs
                        (true, true) => {
                            if y1.is_zero() {
                                return Ok(Point::at_infinity());
                            }

                            let slope = x1
                                .pow(2usize)
                                .fragile_mul(3)?
                                .fragile_add(&curve.a)?
                                .fragile_div(y1.fragile_mul(2)?)?;

                            let x3 = slope.pow(2usize).fragile_sub(x1.fragile_mul(2)?)?;
                            let y3 = slope.fragile_mul(x1.fragile_sub(&x3)?)?.fragile_sub(y1)?;

                            Ok(Point::Normal(x3, y3, curve.to_owned()))
                        }

                        // Different x axis, y axis doesn't matter in this case
                        _ => {
                            let slope = y2.fragile_sub(&y1)?.fragile_div(x2.fragile_sub(&x1)?)?;
                            let x3 = slope.pow(2usize).fragile_sub(x1)?.fragile_sub(x2)?;
                            let y3 = slope.fragile_mul(x1.fragile_sub(&x3)?)?.fragile_sub(y1)?;

                            Ok(Point::Normal(x3, y3, curve.to_owned()))
                        }
                    }
                }
            }
        };

        point.map_err(Into::into)
    }
}

impl<'a, T> FragileMul<BigUint> for &'a Point<T>
where
    T: IsZero + Clone + Eq,
    T: FragileAdd<Ok = T, Error = Error>,
    T: FragileSub<Ok = T, Error = Error>,
    T: FragileMul<Ok = T, Error = Error>,
    T: FragileDiv<Ok = T, Error = Error>,
    T: FragileMul<usize, Ok = T, Error = Error>,
    for<'b> T: FragileAdd<&'b T, Ok = T, Error = Error>,
    for<'b> T: FragileSub<&'b T, Ok = T, Error = Error>,
    for<'b> &'b T: Pow<usize, Output = T>,
    for<'b> &'b T: FragileSub<Ok = T, Error = Error>,
    for<'b> &'b T: FragileMul<usize, Ok = T, Error = Error>,
{
    type Ok = Point<T>;
    type Error = Error;

    fn fragile_mul(self, mut coef: BigUint) -> Result<Self::Ok, Self::Error> {
        let one = BigUint::one();

        let mut result = Point::at_infinity();
        let mut current = self.clone();

        while !coef.is_zero() {
            if &coef & &one == one {
                result = (&result).fragile_add(&current)?;
            }

            coef >>= 1;
            current = current.fragile_add(&current)?;
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
        let curve = EllipticCurve::new(5i32, 7);

        let a = Point::new(-1, -1, curve)?;
        let inf = Point::at_infinity();

        assert_eq!(a.fragile_add(&inf)?, a);
        assert_eq!(inf.fragile_add(&a)?, a);

        Ok(())
    }

    #[test]
    fn addition_with_inverse() -> Result<()> {
        let curve = EllipticCurve::new(5i32, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(-1, 1, curve)?;

        assert_eq!(a.fragile_add(&b)?, Point::at_infinity());
        assert_eq!(b.fragile_add(&a)?, Point::at_infinity());

        Ok(())
    }

    #[test]
    fn addition_diff_points() -> Result<()> {
        let curve = EllipticCurve::new(5i32, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(2, 5, curve)?;
        let c = Point::new(3, -7, curve)?;

        assert_eq!(a.fragile_add(&b)?, c);
        assert_eq!(b.fragile_add(&a)?, c);

        Ok(())
    }

    #[test]
    fn addition_equal_points() -> Result<()> {
        let curve = EllipticCurve::new(5i32, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(18, 77, curve)?;

        assert_eq!(a.fragile_add(&a)?, b);

        Ok(())
    }

    #[test]
    fn addition_with_field_element() -> Result<()> {
        use crate::field::FieldElement;

        let prime = 223usize;
        let curve = EllipticCurve::new(FieldElement::new(0, prime)?, FieldElement::new(7, prime)?);

        let a = Point::new(
            FieldElement::new(192, prime)?,
            FieldElement::new(105, prime)?,
            curve.clone(),
        )?;

        let b = Point::new(
            FieldElement::new(17, prime)?,
            FieldElement::new(56, prime)?,
            curve.clone(),
        )?;

        let c = Point::new(
            FieldElement::new(170, prime)?,
            FieldElement::new(142, prime)?,
            curve,
        )?;

        assert_eq!(c, a.fragile_add(&b)?);
        Ok(())
    }

    #[test]
    fn scalar_multiplication_with_field_element() -> Result<()> {
        use crate::field::FieldElement;

        let prime = 223usize;
        let curve = EllipticCurve::new(FieldElement::new(0, prime)?, FieldElement::new(7, prime)?);

        let a = Point::new(
            FieldElement::new(47, prime)?,
            FieldElement::new(71, prime)?,
            curve.clone(),
        )?;

        let b = Point::new(
            FieldElement::new(47, prime)?,
            FieldElement::new(152, prime)?,
            curve,
        )?;

        assert_eq!(b, a.fragile_mul(BigUint::from(20usize))?);
        assert_eq!(Point::at_infinity(), b.fragile_add(&a)?);
        Ok(())
    }
}
