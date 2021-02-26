use std::ops::{Add, Mul};

use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Pow, Zero};

use crate::utils::prepend_padding;
use crate::{Error, Result};

use super::field::FieldElement;
use super::field::PRIME;

lazy_static! {
    pub(crate) static ref B: FieldElement = FieldElement::new(7usize);
    pub(crate) static ref ECURVE: EllipticCurve =
        EllipticCurve::new(FieldElement::new(0usize), FieldElement::new(7usize));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EllipticCurve {
    a: FieldElement,
    b: FieldElement,
}

impl EllipticCurve {
    pub fn new(a: FieldElement, b: FieldElement) -> Self {
        Self { a, b }
    }

    pub fn contains(&self, x: &FieldElement, y: &FieldElement) -> bool {
        y.pow(2usize) == x.pow(3usize) + &self.a * x + &self.b
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Point {
    AtInfinity,
    Normal(FieldElement, FieldElement),
}

impl Point {
    pub fn new(x: FieldElement, y: FieldElement) -> Result<Self> {
        if ECURVE.contains(&x, &y) {
            Ok(Self::Normal(x, y))
        } else {
            Err(Error::PointNotOnTheCurve)
        }
    }

    pub fn at_infinity() -> Self {
        Self::AtInfinity
    }

    pub fn x(&self) -> Option<&FieldElement> {
        match self {
            Point::AtInfinity { .. } => None,
            Point::Normal(x, _) => Some(x),
        }
    }

    pub fn y(&self) -> Option<&FieldElement> {
        match self {
            Point::AtInfinity { .. } => None,
            Point::Normal(_, y) => Some(y),
        }
    }

    pub fn is_point_at_inf(&self) -> bool {
        matches!(self, Self::AtInfinity)
    }

    /// Serialize the given point with the SEC format
    pub fn serialize(&self, compressed: bool) -> Result<Vec<u8>> {
        match self {
            Self::Normal(x, y) => {
                if compressed {
                    let x_bigendian = prepend_padding(x.0.to_bytes_be(), 32, 0u8)?;
                    let y_evenness = if y.0.is_even() { 0x02 } else { 0x03 };

                    let serialized: Vec<_> =
                        std::iter::once(y_evenness).chain(x_bigendian).collect();

                    Ok(serialized)
                } else {
                    let x_bigendian = prepend_padding(x.0.to_bytes_be(), 32, 0u8)?;
                    let y_bigendian = prepend_padding(y.0.to_bytes_be(), 32, 0u8)?;
                    let serialized: Vec<_> = std::iter::once(0x04u8)
                        .chain(x_bigendian)
                        .chain(y_bigendian)
                        .collect();

                    Ok(serialized)
                }
            }

            _ => Err(Error::SerializePointAtInfinity),
        }
    }

    /// Deserialize the given bytes with the SEC format
    pub fn deserialize<B>(bytes: B) -> Result<Self>
    where
        B: AsRef<[u8]>,
    {
        let bytes = bytes.as_ref();

        let length = bytes.len();
        if length != 33 && length != 65 {
            return Err(Error::InvalidSecBytesLength(length));
        }

        // uncompressed sec format
        if bytes[0] == 0x04 {
            let x = FieldElement::new(BigUint::from_bytes_be(&bytes[1..33]));
            let y = FieldElement::new(BigUint::from_bytes_be(&bytes[33..65]));
            return Self::new(x, y);
        }

        // compressed sec format
        let y_is_even = bytes[0] == 0x02;
        let x = FieldElement::new(BigUint::from_bytes_be(&bytes[1..]));

        // elliptic curve equation: y^2 = x^3 + x*a + b
        // rhs of the elliptic curve equation (note a = 0)
        let alpha = x.pow(3u8) + &*B;

        // solve lhs
        let beta = alpha.sqrt();

        let y = match (beta.0.is_even(), y_is_even) {
            (true, true) | (false, false) => beta,
            (true, false) | (false, true) => FieldElement::new(&*PRIME - beta.0),
        };

        Ok(Self::Normal(x, y)) // no need to check
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Point::AtInfinity
    }

    fn is_zero(&self) -> bool {
        matches!(self, Point::AtInfinity)
    }
}

impl<'a, 'b> Add<&'a Point> for &'b Point {
    type Output = Point;

    fn add(self, rhs: &'a Point) -> Self::Output {
        match (self, rhs) {
            // Additive identity
            (Point::AtInfinity, p) | (p, Point::AtInfinity) => p.to_owned(),

            // Normal addition between points
            (Point::Normal(x1, y1), Point::Normal(x2, y2)) => match (x1 == x2, y1 == y2) {
                // Same x axis, rhs is additive inverse of self and viceversa
                (true, false) => Point::at_infinity(),

                // Same x and y axis, self is equal to rhs
                (true, true) => {
                    if y1.is_zero() {
                        return Point::at_infinity();
                    }

                    let slope = (x1.pow(2usize) * 3usize + &ECURVE.a) / (y1 * 2usize);
                    let x3 = slope.pow(2usize) - (x1 * 2);
                    let y3 = slope * (x1 - &x3) - y1;

                    Point::Normal(x3, y3)
                }

                // Different x axis, y axis doesn't matter in this case
                _ => {
                    let slope = (y2 - y1) / (x2 - x1);
                    let x3 = slope.pow(2usize) - x1 - x2;
                    let y3 = slope * (x1 - &x3) - y1;

                    Point::Normal(x3, y3)
                }
            },
        }
    }
}

impl<'a, U> Mul<U> for &'a Point
where
    U: Into<BigUint>,
{
    type Output = Point;

    fn mul(self, coef: U) -> Self::Output {
        let mut coef = coef.into() % &*PRIME;

        let one = BigUint::one();
        let mut result = Point::zero();
        let mut current = self.clone();

        while !coef.is_zero() {
            if &coef & &one == one {
                result = &result + &current;
            }

            coef >>= 1;
            current = &current + &current;
        }

        result
    }
}

impl<U> Mul<U> for Point
where
    U: Into<BigUint>,
{
    type Output = Point;

    fn mul(self, coef: U) -> Self::Output {
        Mul::mul(&self, coef)
    }
}

forward_binop_impl!(for non-copyable Point where Add does add);
