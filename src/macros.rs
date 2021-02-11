macro_rules! forward_binop_impl {
    (for non-copyable $type:ty where $binop:ident does $met:ident) => {
        impl<'a> $binop<&'a $type> for $type {
            type Output = $type;

            fn $met(self, rhs: &'a $type) -> Self::Output {
                $binop::$met(&self, rhs)
            }
        }

        impl<'a> $binop<$type> for &'a $type {
            type Output = $type;

            fn $met(self, rhs: $type) -> Self::Output {
                $binop::$met(self, &rhs)
            }
        }

        impl $binop for $type {
            type Output = $type;

            fn $met(self, rhs: $type) -> Self::Output {
                $binop::$met(&self, &rhs)
            }
        }
    };
}

macro_rules! biguint {
    ($hex:tt) => {
        num_bigint::BigUint::from_bytes_be(&hex_literal::hex!($hex))
    };
}

macro_rules! field_elem {
    ($hex:tt) => {{
        use $crate::secp256k1::field::FieldElement;
        FieldElement::new(biguint!($hex))
    }};
}

macro_rules! ec_point {
    ($hex_x:tt, $hex_y:tt,) => {{
        use $crate::secp256k1::curve::Point;
        Point::new(field_elem!($hex_x), field_elem!($hex_y)).unwrap()
    }};
}
