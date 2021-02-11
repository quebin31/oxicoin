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
