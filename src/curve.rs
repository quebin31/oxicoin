use std::ops::Add;

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ECPoint {
    a: isize,
    b: isize,
    x: Option<isize>,
    y: Option<isize>,
}

impl ECPoint {
    pub fn new<U>(x: U, y: U, a: isize, b: isize) -> Result<Self>
    where
        U: Into<Option<isize>>,
    {
        let x = x.into();
        let y = y.into();

        match (x, y) {
            (None, None) => Ok(Self { a, b, x, y }),
            (Some(x), Some(y)) => {
                if y.pow(2) != x.pow(3) + a * x + b {
                    Ok(Self {
                        a,
                        b,
                        x: Some(x),
                        y: Some(y),
                    })
                } else {
                    Err(Error::PointNotInTheCurve(x, y))
                }
            }

            _ => Err(Error::InvalidECPoint),
        }
    }

    pub fn same_curve(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Add for ECPoint {
    type Output = Result<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if !self.same_curve(&rhs) {
            return Err(Error::PointsNotInTheSameCurve);
        }

        if self.x.is_none() && self.y.is_none() {
            return Ok(rhs);
        }

        if rhs.x.is_none() && rhs.y.is_none() {
            return Ok(self);
        }

        if self.x == rhs.x {
            return Ok(Self {
                x: None,
                y: None,
                ..self
            });
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn not_in_curve() {
        let res = ECPoint::new(-1, -2, 5, 7);
        assert!(res.is_err());
    }

    #[test]
    fn equality() -> Result<()> {
        let a = ECPoint::new(-1, -1, 5, 7)?;
        let b = ECPoint::new(18, 77, 5, 7)?;

        assert_eq!(a, a);
        assert_ne!(a, b);

        Ok(())
    }
}
