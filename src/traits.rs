use std::ops::{Add, Div, Mul, Sub};

use num_traits::Zero;

use crate::Infallible;

pub trait IsZero {
    fn is_zero(&self) -> bool;
}

impl<T> IsZero for T
where
    T: Zero,
{
    fn is_zero(&self) -> bool {
        <T as Zero>::is_zero(&self)
    }
}

macro_rules! define_may_traits {
    ($($trait:ident, $met:ident,)*) => {
        $(
            pub trait $trait<Rhs = Self> {
                type Output;
                type Error;

                fn $met(self, other: Rhs) -> Result<Self::Output, Self::Error>;
            }
        )*
    };
}

macro_rules! prim_impl_may_traits {
    ($type:ty) => {
        prim_impl_may_traits! {
            MayAdd, may_add, $type, add,
            MaySub, may_sub, $type, sub,
            MayMul, may_mul, $type, mul,
            MayDiv, may_div, $type, div,
        }
    };

    ($($trait:ident, $tmet: ident, $type:ty, $pmet:ident,)*) => {
        $(
            impl $trait for $type
            {
                type Output = $type;
                type Error = Infallible;

                fn $tmet(self, other: $type) -> Result<Self::Output, Self::Error> {
                    Ok(self.$pmet(other))
                }
            }
        )*
    };
}

macro_rules! blanket_impl_may_result {
    ($($trait:ident, $met:ident,)*) => {
        $(
            impl<T, E> $trait<T> for Result<T, E>
            where
                T: $trait<Output = T, Error = E>
            {
                type Output = T;
                type Error = E;

                fn $met(self, other: T) -> Result<Self::Output, Self::Error> {
                    match self {
                        Ok(val) => val.$met(other),
                        Err(e) => Err(e)
                    }
                }
            }

            impl<T, E> $trait<Result<T, E>> for T
            where
                T: $trait<Output = T, Error = E>
            {
                type Output = T;
                type Error = E;

                fn $met(self, other: Result<T, E>) -> Result<Self::Output, Self::Error> {
                    match other {
                        Ok(val) => self.$met(val),
                        Err(e) => Err(e)
                    }
                }
            }

            impl<T, E> $trait for Result<T, E>
            where
                T: $trait<Output = T, Error = E>
            {
                type Output = T;
                type Error = E;

                fn $met(self, other: Result<T, E>) -> Result<Self::Output, Self::Error> {
                    match (self, other) {
                        (Ok(a), Ok(b)) => a.$met(b),
                        (Err(e), _) | (_, Err(e)) => Err(e),
                    }
                }
            }
        )*
    };
}

macro_rules! prim_impl_may_mul_usize_trait {
    ($($type:ty,)*) => {
        $(
            impl MayMul<usize> for $type {
                type Output = $type;
                type Error = Infallible;

                fn may_mul(self, other: usize) -> Result<Self::Output, Self::Error> {
                    Ok(self * other as $type)
                }
            }

        )*
    };
}

define_may_traits! {
    MayAdd, may_add,
    MaySub, may_sub,
    MayMul, may_mul,
    MayDiv, may_div,
}

blanket_impl_may_result! {
    MayAdd, may_add,
    MaySub, may_sub,
    MayMul, may_mul,
    MayDiv, may_div,
}

prim_impl_may_traits!(u8);
prim_impl_may_traits!(i8);
prim_impl_may_traits!(u16);
prim_impl_may_traits!(i16);
prim_impl_may_traits!(u32);
prim_impl_may_traits!(i32);
prim_impl_may_traits!(u64);
prim_impl_may_traits!(i64);
prim_impl_may_traits!(usize);
prim_impl_may_traits!(isize);

prim_impl_may_mul_usize_trait! {
    i32,
    u32,
    isize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Infallible;
    use anyhow::Error;

    #[test]
    fn add_on_result() -> Result<(), Error> {
        let a: Result<_, Infallible> = Ok(3);
        assert_eq!(8, a.may_add(5)?);
        Ok(())
    }

    #[test]
    fn add_with_result() -> Result<(), Error> {
        let a: Result<_, Infallible> = Ok(4);
        assert_eq!(7, 3.may_add(a)?);
        Ok(())
    }

    #[test]
    fn add_both_result() -> Result<(), Error> {
        let a: Result<_, Infallible> = Ok(3);
        let b: Result<_, Infallible> = Ok(6);
        assert_eq!(9, a.may_add(b)?);
        Ok(())
    }

    #[test]
    fn add_chained() -> Result<(), Error> {
        let a = 3.may_add(4).may_add(5)?;
        assert_eq!(12, a);
        Ok(())
    }
}
