use std::ops::{Add, Mul};

use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::{One, Pow, Zero};

use crate::{forward_binop_impl, Error};

use super::field::FieldElement;
use super::field::PRIME;

lazy_static! {
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
    pub fn new(x: FieldElement, y: FieldElement) -> Result<Self, Error> {
        if ECURVE.contains(&x, &y) {
            Ok(Self::Normal(x, y))
        } else {
            Err(Error::PointNotInTheCurve)
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
