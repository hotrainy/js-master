/// Implements a test on a unary operation using proptest.
///
/// Compares the vector operation to the equivalent scalar operation.
#[macro_export]
macro_rules! impl_unary_op_test {
    { $scalar:ty, $trait:ident :: $fn:ident, $scalar_fn:expr } => {
        test_helpers::test_lanes! {
            fn $fn<const LANES: usize>() {
                test_helpers::test_unary_elementwise(
                    &<core_simd::simd::Simd<$scalar, LANES> as core::ops::$trait>::$fn,
                    &$scalar_fn,
                    &|_| true,
                );
            }
        }
    };
    { $scalar:ty, $trait:ident :: $fn:ident } => {
        impl_unary_op_test! { $scalar, $trait::$fn, <$scalar as core::ops::$trait>::$fn }
    };
}

/// Implements a test on a binary operation using proptest.
///
/// Compares the vector operation to the equivalent scalar operation.
#[macro_export]
macro_rules! impl_binary_op_test {
    { $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident, $scalar_fn:expr } => {
        mod $fn {
            use super::*;
            use core_simd::simd::Simd;

            test_helpers::test_lanes! {
                fn normal<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &<Simd<$scalar, LANES> as core::ops::$trait>::$fn,
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }

                fn assign<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &|mut a, b| { <Simd<$scalar, LANES> as core::ops::$trait_assign>::$fn_assign(&mut a, b); a },
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }
            }
        }
    };
    { $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident } => {
        impl_binary_op_test! { $scalar, $trait::$fn, $trait_assign::$fn_assign, <$scalar as core::ops::$trait>::$fn }
    };
}

/// Implements a test on a binary operation using proptest.
///
/// Like `impl_binary_op_test`, but allows providing a function for rejecting particular inputs
/// (like the `proptest_assume` macro).
///
/// Compares the vector operation 