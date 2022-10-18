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
/// Compares the vector operation to the equivalent scalar operation.
#[macro_export]
macro_rules! impl_binary_checked_op_test {
    { $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident, $scalar_fn:expr, $check_fn:expr } => {
        mod $fn {
            use super::*;
            use core_simd::simd::Simd;

            test_helpers::test_lanes! {
                fn normal<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &<Simd<$scalar, LANES> as core::ops::$trait>::$fn,
                        &$scalar_fn,
                        &|x, y| x.iter().zip(y.iter()).all(|(x, y)| $check_fn(*x, *y)),
                    );
                }

                fn assign<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &|mut a, b| { <Simd<$scalar, LANES> as core::ops::$trait_assign>::$fn_assign(&mut a, b); a },
                        &$scalar_fn,
                        &|x, y| x.iter().zip(y.iter()).all(|(x, y)| $check_fn(*x, *y)),
                    )
                }
            }
        }
    };
    { $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident, $check_fn:expr } => {
        impl_binary_checked_op_test! { $scalar, $trait::$fn, $trait_assign::$fn_assign, <$scalar as core::ops::$trait>::$fn, $check_fn }
    };
}

#[macro_export]
macro_rules! impl_common_integer_tests {
    { $vector:ident, $scalar:ident } => {
        test_helpers::test_lanes! {
            fn reduce_sum<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_sum(),
                        x.iter().copied().fold(0 as $scalar, $scalar::wrapping_add),
                    );
                    Ok(())
                });
            }

            fn reduce_product<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_product(),
                        x.iter().copied().fold(1 as $scalar, $scalar::wrapping_mul),
                    );
                    Ok(())
                });
            }

            fn reduce_and<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_and(),
                        x.iter().copied().fold(-1i8 as $scalar, <$scalar as core::ops::BitAnd>::bitand),
                    );
                    Ok(())
                });
            }

            fn reduce_or<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_or(),
                        x.iter().copied().fold(0 as $scalar, <$scalar as core::ops::BitOr>::bitor),
                    );
                    Ok(())
                });
            }

            fn reduce_xor<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_xor(),
                        x.iter().copied().fold(0 as $scalar, <$scalar as core::ops::BitXor>::bitxor),
                    );
                    Ok(())
                });
            }

            fn reduce_max<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_max(),
                        x.iter().copied().max().unwrap(),
                    );
                    Ok(())
                });
            }

            fn reduce_min<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).reduce_min(),
                        x.iter().copied().min().unwrap(),
                    );
                    Ok(())
                });
            }
        }
    }
}

/// Implement tests for signed integers.
#[macro_export]
macro_rules! impl_signed_tests {
    { $scalar:tt } => {
        mod $scalar {
            use core_simd::simd::SimdInt;
            type Vector<const LANES: usize> = core_simd::simd::Simd<Scalar, LANES>;
            type Scalar = $scalar;

            impl_common_integer_tests! { Vector, Scalar }

            test_helpers::test_lanes! {
                fn neg<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &<Vector::<LANES> as core::ops::Neg>::neg,
                        &<Scalar as core::ops::Neg>::neg,
                        &|x| !x.contains(&Scalar::MIN),
                    );
               