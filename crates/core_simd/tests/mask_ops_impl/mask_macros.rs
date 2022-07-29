macro_rules! mask_tests {
    { $vector:ident, $lanes:literal } => {
        #[cfg(test)]
        mod $vector {
            use core_simd::simd::$vector as Vector;
            const LANES: usize = $lanes;

            #[cfg(target_arch = "wasm32")]
            use wasm_bindgen_test::*;

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_test_configure!(run_in_browser);

            fn from_slice(slice: &[bool]) -> Vector {
                let mut value = Vector::default();
                for (i, b) in slice.iter().take(LANES).enumerate() {
                    value.set(i, *b);
                }
                value
            }

            fn apply_unary_lanewise(x: Vector, f: impl Fn(bool) -> bool) -> Vector {
                let mut value = Vector::default();
                for i in 0..LANES {
                    value.set(i, f(x.test(i)));
                }
                value
            }

            fn apply_binary_lanewise(x: Vector, y: Vector, f: impl Fn(bool, bool) -> bool) -> Vector {
                let mut value = Vector::default();
                for i in 0..LANES {
                    value.set(i, f(x.test(i), y.test(i)));
                }
                value
            }

            fn apply_binary_scalar_lhs_lanewise(x: bool, mut y: Vector, f: impl Fn(bool, bool) -> bool) -> Vector {
                for i in 0..LANES {
                    y.set(i, f(x, y.test(i)));
                }
                y
            }

            fn apply_binary_scalar_rhs_lanewise(mut x: Vector, y: bool, f: impl Fn(bool, bool) -> bool) -> Vector {
                for i in 0..LANES {
                    x.set(i, f(x.test(i), y));
                }
                x
            }

            const A: [bool; 64] = [
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
                false, true, false, true, false, false, true, true,
            ];
            const B: [bool; 64] = [
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
                false, false, true, true, false, true, false, true,
            ];

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn bitand() {
                let a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::BitAnd::bitand);
                assert_eq!(a & b, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn bitand_assign() {
                let mut a = from_slice(&A);
                let b = from_slice(&B);
                let expected = apply_binary_lanewise(a, b, core::ops::BitAnd::bitand);
                a &= b;
                assert_eq!(a, expected);
            }

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn bitand_scalar_rhs() {
                let a = from_slice(&A);
                let expected = a;
                assert_eq!(a & true, expected);
                assert_eq!(a & false, Vector::splat(false));
            }

            #[te