pub mod zero {
    use num_traits::Zero;

    /// Check wether self is zero, more relaxed than `num_traits::Zero`
    pub trait IsZero {
        fn is_zero(&self) -> bool;
    }

    /// Implement for all `T` that already implement `num_traits::Zero`
    impl<T> IsZero for T
    where
        T: Zero,
    {
        fn is_zero(&self) -> bool {
            <T as Zero>::is_zero(&self)
        }
    }
}

pub mod fragile {
    use std::convert::Infallible;
    use std::ops::{Add, Div, Mul, Sub};

    use num_bigint::{BigInt, BigUint};

    macro_rules! fragile_traits {
        ($($trait:ident does $met:ident,)*) => {
            $(
                pub trait $trait<Rhs = Self> {
                    type Ok;
                    type Error;

                    fn $met(self, rhs: Rhs) -> Result<Self::Ok, Self::Error>;
                }
            )*
        };
    }

    fragile_traits! {
        FragileAdd does fragile_add,
        FragileSub does fragile_sub,
        FragileMul does fragile_mul,
        FragileDiv does fragile_div,
    }

    macro_rules! impl_fragile {
        (for non-fragile $type:ty) => {
            impl_fragile! {
                for copyable
                FragileAdd, fragile_add, $type, add,
                FragileSub, fragile_sub, $type, sub,
                FragileMul, fragile_mul, $type, mul,
                FragileDiv, fragile_div, $type, div,
            }
        };

        (for non-fragile non-copyable $type:ty ) => {
            impl_fragile! {
                for non-copyable
                FragileAdd, fragile_add, $type, add,
                FragileSub, fragile_sub, $type, sub,
                FragileMul, fragile_mul, $type, mul,
                FragileDiv, fragile_div, $type, div,
            }
        };

        (for copyable $($trait:ident, $trait_met: ident, $type:ty, $type_met:ident,)*) => {
            $(
                impl $trait for $type {
                    type Ok = $type;
                    type Error = Infallible;

                    #[inline]
                    fn $trait_met(self, rhs: $type) -> Result<Self::Ok, Self::Error> {
                        Ok(self.$type_met(rhs))
                    }
                }

                forward_fragile_impl!(for copyable $type => $trait, $trait_met, Infallible);
            )*
        };

        (for non-copyable $($trait:ident, $trait_met: ident, $type:ty, $type_met:ident,)*) => {
            $(
                impl<'a, 'b> $trait<&'a $type> for &'b $type {
                    type Ok = $type;
                    type Error = Infallible;

                    #[inline]
                    fn $trait_met(self, rhs: &'a $type) -> Result<Self::Ok, Self::Error> {
                        Ok(self.$type_met(rhs))
                    }
                }

                forward_fragile_impl!(for non-copyable $type => $trait, $trait_met, Infallible);
            )*
        };
    }

    #[macro_export]
    macro_rules! forward_fragile_impl {
        (for copyable $type:ty => $trait:ident, $trait_met: ident, $err:path) => {
            impl<'a> $trait<$type> for &'a $type {
                type Ok = $type;
                type Error = $err;

                #[inline]
                fn $trait_met(self, rhs: $type) -> Result<Self::Ok, Self::Error> {
                    $trait::$trait_met(*self, rhs)
                }
            }

            impl<'a> $trait<&'a $type> for $type {
                type Ok = $type;
                type Error = $err;

                #[inline]
                fn $trait_met(self, rhs: &'a $type) -> Result<Self::Ok, Self::Error> {
                    $trait::$trait_met(self, *rhs)
                }
            }

            impl<'a, 'b> $trait<&'a $type> for &'b $type {
                type Ok = $type;
                type Error = $err;

                #[inline]
                fn $trait_met(self, rhs: &'a $type) -> Result<Self::Ok, Self::Error> {
                    $trait::$trait_met(*self, *rhs)
                }
            }
        };

        (for non-copyable $type:ty => $trait:ident, $trait_met: ident, $err:path) => {
            impl<'a> $trait<&'a $type> for $type {
                type Ok = $type;
                type Error = $err;

                #[inline]
                fn $trait_met(self, rhs: &'a $type) -> Result<Self::Ok, Self::Error> {
                    $trait::$trait_met(&self, rhs)
                }
            }

            impl<'a> $trait<$type> for &'a $type {
                type Ok = $type;
                type Error = $err;

                #[inline]
                fn $trait_met(self, rhs: $type) -> Result<Self::Ok, Self::Error> {
                    $trait::$trait_met(self, &rhs)
                }
            }

            impl $trait for $type {
                type Ok = $type;
                type Error = $err;

                #[inline]
                fn $trait_met(self, rhs: $type) -> Result<Self::Ok, Self::Error> {
                    $trait::$trait_met(&self, &rhs)
                }
            }
        };
    }

    impl_fragile!(for non-fragile i32);
    impl_fragile!(for non-fragile u32);
    impl_fragile!(for non-fragile i64);
    impl_fragile!(for non-fragile u64);
    impl_fragile!(for non-fragile isize);
    impl_fragile!(for non-fragile usize);
    impl_fragile!(for non-fragile non-copyable BigInt);
    impl_fragile!(for non-fragile non-copyable BigUint);

    macro_rules! impl_scalar_fragile_mul {
        (for non-fragile $type:ty) => {
            impl FragileMul<usize> for $type {
                type Ok = $type;
                type Error = Infallible;

                fn fragile_mul(self, rhs: usize) -> Result<Self::Ok, Self::Error> {
                    Ok(self * rhs as $type)
                }
            }

            impl<'a> FragileMul<usize> for &'a $type {
                type Ok = $type;
                type Error = Infallible;

                fn fragile_mul(self, rhs: usize) -> Result<Self::Ok, Self::Error> {
                    FragileMul::fragile_mul(*self, rhs)
                }
            }
        };

        (for non-fragile non-copyable $type:ty) => {
            impl<'a> FragileMul<usize> for &'a $type {
                type Ok = $type;
                type Error = Infallible;

                fn fragile_mul(self, rhs: usize) -> Result<Self::Ok, Self::Error> {
                    let rhs: $type = rhs.into();
                    FragileMul::fragile_mul(self, &rhs)
                }
            }

            impl FragileMul<usize> for $type {
                type Ok = $type;
                type Error = Infallible;

                fn fragile_mul(self, rhs: usize) -> Result<Self::Ok, Self::Error> {
                    FragileMul::fragile_mul(&self, rhs)
                }
            }
        };
    }

    impl_scalar_fragile_mul!(for non-fragile i32);
    impl_scalar_fragile_mul!(for non-fragile u32);
    impl_scalar_fragile_mul!(for non-fragile i64);
    impl_scalar_fragile_mul!(for non-fragile u64);
    impl_scalar_fragile_mul!(for non-fragile isize);
    impl_scalar_fragile_mul!(for non-fragile non-copyable BigInt);
    impl_scalar_fragile_mul!(for non-fragile non-copyable BigUint);
}
