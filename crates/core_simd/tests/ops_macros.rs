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
                }

                fn is_positive<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_positive,
                        &Scalar::is_positive,
                        &|_| true,
                    );
                }

                fn is_negative<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_negative,
                        &Scalar::is_negative,
                        &|_| true,
                    );
                }

                fn signum<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::signum,
                        &Scalar::signum,
                        &|_| true,
                    )
                }

                fn div_min_may_overflow<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(Scalar::MIN);
                    let b = Vector::<LANES>::splat(-1);
                    assert_eq!(a / b, a);
                }

                fn rem_min_may_overflow<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(Scalar::MIN);
                    let b = Vector::<LANES>::splat(-1);
                    assert_eq!(a % b, Vector::<LANES>::splat(0));
                }

                fn simd_min<const LANES: usize>() {
                    use core_simd::simd::SimdOrd;
                    let a = Vector::<LANES>::splat(Scalar::MIN);
                    let b = Vector::<LANES>::splat(0);
                    assert_eq!(a.simd_min(b), a);
                    let a = Vector::<LANES>::splat(Scalar::MAX);
                    let b = Vector::<LANES>::splat(0);
                    assert_eq!(a.simd_min(b), b);
                }

                fn simd_max<const LANES: usize>() {
                    use core_simd::simd::SimdOrd;
                    let a = Vector::<LANES>::splat(Scalar::MIN);
                    let b = Vector::<LANES>::splat(0);
                    assert_eq!(a.simd_max(b), b);
                    let a = Vector::<LANES>::splat(Scalar::MAX);
                    let b = Vector::<LANES>::splat(0);
                    assert_eq!(a.simd_max(b), a);
                }

                fn simd_clamp<const LANES: usize>() {
                    use core_simd::simd::SimdOrd;
                    let min = Vector::<LANES>::splat(Scalar::MIN);
                    let max = Vector::<LANES>::splat(Scalar::MAX);
                    let zero = Vector::<LANES>::splat(0);
                    let one = Vector::<LANES>::splat(1);
                    let negone = Vector::<LANES>::splat(-1);
                    assert_eq!(zero.simd_clamp(min, max), zero);
                    assert_eq!(zero.simd_clamp(min, one), zero);
                    assert_eq!(zero.simd_clamp(one, max), one);
                    assert_eq!(zero.simd_clamp(min, negone), negone);
                }
            }

            test_helpers::test_lanes_panic! {
                fn div_by_all_zeros_panics<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(0);
                    let _ = a / b;
                }

                fn div_by_one_zero_panics<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let mut b = Vector::<LANES>::splat(21);
                    b[0] = 0 as _;
                    let _ = a / b;
                }

                fn rem_zero_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(0);
                    let _ = a % b;
                }
            }

            test_helpers::test_lanes! {
                fn div_neg_one_no_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(-1);
                    let _ = a / b;
                }

                fn rem_neg_one_no_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(-1);
                    let _ = a % b;
                }
            }

            impl_binary_op_test!(Scalar, Add::add, AddAssign::add_assign, Scalar::wrapping_add);
            impl_binary_op_test!(Scalar, Sub::sub, SubAssign::sub_assign, Scalar::wrapping_sub);
            impl_binary_op_test!(Scalar, Mul::mul, MulAssign::mul_assign, Scalar::wrapping_mul);

            // Exclude Div and Rem panicking cases
            impl_binary_checked_op_test!(Scalar, Div::div, DivAssign::div_assign, Scalar::wrapping_div, |x, y| y != 0 && !(x == Scalar::MIN && y == -1));
            impl_binary_checked_op_test!(Scalar, Rem::rem, RemAssign::rem_assign, Scalar::wrapping_rem, |x, y| y != 0 && !(x == Scalar::MIN && y == -1));

            impl_unary_op_test!(Scalar, Not::not);
            impl_binary_op_test!(Scalar, BitAnd::bitand, BitAndAssign::bitand_assign);
            impl_binary_op_test!(Scalar, BitOr::bitor, BitOrAssign::bitor_assign);
            impl_binary_op_test!(Scalar, BitXor::bitxor, BitXorAssign::bitxor_assign);
        }
    }
}

/// Implement tests for unsigned integers.
#[macro_export]
macro_rules! impl_unsigned_tests {
    { $scalar:tt } => {
        mod $scalar {
            use core_simd::simd::SimdUint;
            type Vector<const LANES: usize> = core_simd::simd::Simd<Scalar, LANES>;
            type Scalar = $scalar;

            impl_common_integer_tests! { Vector, Scalar }

            test_helpers::test_lanes_panic! {
                fn rem_zero_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(0);
                    let _ = a % b;
                }
            }

            impl_binary_op_test!(Scalar, Add::add, AddAssign::add_assign, Scalar::wrapping_add);
            impl_binary_op_test!(Scalar, Sub::sub, SubAssign::sub_assign, Scalar::wrapping_sub);
            impl_binary_op_test!(Scalar, Mul::mul, MulAssign::mul_assign, Scalar::wrapping_mul);

            // Exclude Div and Rem panicking cases
            impl_binary_checked_op_test!(Scalar, Div::div, DivAssign::div_assign, Scalar::wrapping_div, |_, y| y != 0);
            impl_binary_checked_op_test!(Scalar, Rem::rem, RemAssign::rem_assign, Scalar::wrapping_rem, |_, y| y != 0);

            impl_unary_op_test!(Scalar, Not::not);
            impl_binary_op_test!(Scalar, BitAnd::bitand, BitAndAssign::bitand_assign);
            impl_binary_op_test!(Scalar, BitOr::bitor, BitOrAssign::bitor_assign);
            impl_binary_op_test!(Scalar, BitXor::bitxor, BitXorAssign::bitxor_assign);
        }
    }
}

/// Implement tests for floating point numbers.
#[macro_export]
macro_rules! impl_float_tests {
    { $scalar:tt, $int_scalar:tt } => {
        mod $scalar {
            use core_simd::simd::SimdFloat;
            type Vector<const LANES: usize> = core_simd::simd::Simd<Scalar, LANES>;
            type Scalar = $scalar;

            impl_unary_op_test!(Scalar, Neg::neg);
            impl_binary_op_test!(Scalar, Add::add, AddAssign::add_assign);
            impl_binary_op_test!(Scalar, Sub::sub, SubAssign::sub_assign);
            impl_binary_op_test!(Scalar, Mul::mul, MulAssign::mul_assign);
            impl_binary_op_test!(Scalar, Div::div, DivAssign::div_assign);
            impl_binary_op_test!(Scalar, Rem::rem, RemAssign::rem_assign);

            test_helpers::test_lanes! {
                fn is_sign_positive<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_sign_positive,
                        &Scalar::is_sign_positive,
                        &|_| true,
                    );
                }

                fn is_sign_negative<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_sign_negative,
                        &Scalar::is_sign_negative,
                        &|_| true,
                    );
                }

                fn is_finite<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_finite,
                        &Scalar::is_finite,
                        &|_| true,
                    );
                }

                fn is_infinite<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_infinite,
                        &Scalar::is_infinite,
                        &|_| true,
                    );
                }

                fn is_nan<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_nan,
                        &Scalar::is_nan,
                        &|_| true,
                    );
                }

                fn is_normal<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_normal,
                        &Scalar::is_normal,
                        &|_| true,
                    );
                }

                fn is_subnormal<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_subnormal,
                        &Scalar::is_subnormal,
                        &|_| true,
                    );
                }

                fn abs<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::abs,
                        &Scalar::abs,
                        &|_| true,
                    )
                }

                fn recip<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::recip,
                        &Scalar::recip,
                        &|_| true,
                    )
                }

                fn to_degrees<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::to_degrees,
                        &Scalar::to_degrees,
                        &|_| true,
                    )
                }

                fn to_radians<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::to_radians,
                        &Scalar::to_radians,
                        &|_| true,
                    )
                }

                fn signum<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::signum,
                        &Scalar::signum,
                        &|_| true,
                    )
                }

                fn copysign<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &Vector::<LANES>::copysign,
                        &Scalar::copysign,
                        &|_, _| true,
                    )
                }

                fn simd_min<const LANES: usize>() {
                    // Regular conditions (both values aren't zero)
                    test_helpers::test_binary_elementwise(
                        &Vector::<LANES>::simd_min,
                        &Scalar::min,
                        // Reject the case where both values are zero with different signs
                        &|a, b| {
                            for (a, b) in a.iter().zip(b.iter()) {
                                if *a == 0. && *b == 0. && a.signum() != b.signum() {
                                    return false;
                                }
                            }
                            true
                        }
                    );

                    // Special case where both values are zero
                    let p_zero = Vector::<LANES>::splat(0.);
                    let n_zero = Vector::<LANES>::splat(-0.);
                    assert!(p_zero.simd_min(n_zero).to_array().iter().all(|x| *x == 0.));
                    assert!(n_zero.simd_min(p_zero).to_array().iter().all(|x| *x == 0.));
                }

                fn simd_max<const LANES: usize>() {
                    // Regular conditions (both values aren't zero)
                    test_helpers::test_binary_elementwise(
                        &Vector::<LANES>::simd_max,
                        &Scalar::max,
                        // Reject the case where both values are 