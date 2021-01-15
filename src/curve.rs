use std::ops::Add;

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EllipticCurve {
    a: isize,
    b: isize,
}

impl EllipticCurve {
    pub fn new(a: isize, b: isize) -> Self {
        Self { a, b }
    }

    pub fn contains(&self, x: isize, y: isize) -> bool {
        y * y == (x * x * x) + self.a * x + self.b
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    AtInfinity {
        curve: EllipticCurve,
    },

    Normal {
        x: isize,
        y: isize,
        curve: EllipticCurve,
    },
}

impl Point {
    pub fn new(x: isize, y: isize, curve: EllipticCurve) -> Result<Self> {
        if curve.contains(x, y) {
            Ok(Self::Normal { x, y, curve })
        } else {
            Err(Error::PointNotInTheCurve(x, y))
        }
    }

    pub fn at_inf(curve: EllipticCurve) -> Self {
        Self::AtInfinity { curve }
    }

    pub fn x(&self) -> Option<isize> {
        match *self {
            Point::AtInfinity { .. } => None,
            Point::Normal { x, .. } => Some(x),
        }
    }

    pub fn y(&self) -> Option<isize> {
        match *self {
            Point::AtInfinity { .. } => None,
            Point::Normal { y, .. } => Some(y),
        }
    }

    pub fn curve(&self) -> EllipticCurve {
        match *self {
            Point::AtInfinity { curve } => curve,
            Point::Normal { curve, .. } => curve,
        }
    }

    pub fn same_curve(&self, other: &Self) -> bool {
        self.curve() == other.curve()
    }

    pub fn is_point_at_inf(&self) -> bool {
        matches!(self, Self::AtInfinity { .. })
    }
}

impl Add for Point {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if !self.same_curve(&rhs) {
            return Err(Error::PointsNotInTheSameCurve);
        }

        match (self, rhs) {
            (Self::AtInfinity { .. }, _) => Ok(rhs),
            (_, Self::AtInfinity { .. }) => Ok(self),
            (
                Self::Normal {
                    x: x1,
                    y: y1,
                    curve,
                },
                Self::Normal { x: x2, y: y2, .. },
            ) => match (x1 == x2, y1 == y2) {
                // Same x axis, rhs is additive inverse of self and viceversa
                (true, false) => Ok(Self::at_inf(curve)),

                // Same x and y axis, self is equal to rhs
                (true, true) => {
                    if y1 == 0 {
                        return Ok(Self::at_inf(curve));
                    }

                    let slope = (3 * x1 * x1 + curve.a) / (2 * y1);
                    let x3 = slope * slope - 2 * x1;
                    let y3 = slope * (x1 - x3) - y1;

                    Self::new(x3, y3, curve)
                }

                // Different x axis, y axis doesn't matter in this case
                _ => {
                    let slope = (y2 - y1) / (x2 - x1);
                    let x3 = slope * slope - x1 - x2;
                    let y3 = slope * (x1 - x3) - y1;

                    Self::new(x3, y3, curve)
                }
            },
        }
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

        assert_eq!(a, a);
        assert_ne!(a, b);

        Ok(())
    }

    #[test]
    fn addition_with_inf() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let inf = Point::at_inf(curve);

        assert_eq!(a.add(inf)?, a);
        assert_eq!(inf.add(a)?, a);

        Ok(())
    }

    #[test]
    fn addition_with_inverse() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(-1, 1, curve)?;

        assert_eq!(a.add(b)?, Point::at_inf(curve));
        assert_eq!(b.add(a)?, Point::at_inf(curve));

        Ok(())
    }

    #[test]
    fn addition_diff_points() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(2, 5, curve)?;
        let c = Point::new(3, -7, curve)?;

        assert_eq!(a.add(b)?, c);
        assert_eq!(b.add(a)?, c);

        Ok(())
    }

    #[test]
    fn addition_equal_points() -> Result<()> {
        let curve = EllipticCurve::new(5, 7);
        let a = Point::new(-1, -1, curve)?;
        let b = Point::new(18, 77, curve)?;

        assert_eq!(a.add(a)?, b);

        Ok(())
    }
}
