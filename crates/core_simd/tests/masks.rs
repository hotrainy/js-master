
#![feature(portable_simd)]

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

macro_rules! test_mask_api {
    { $type:ident } => {
        #[allow(non_snake_case)]
        mod $type {
            #[cfg(target_arch = "wasm32")]
            use wasm_bindgen_test::*;

            use core_simd::simd::Mask;

            #[test]
            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
            fn set_and_test() {
                let values = [true, false, false, true, false, false, true, false];
                let mut mask = Mask::<$type, 8>::splat(false);
                for (lane, value) in values.iter().copied().enumerate() {
                    mask.set(lane, value);
                }
                for (lane, value) in values.iter().copied().enumerate() {
                    assert_eq!(mask.test(lane), value);
                }
            }

            #[test]
            #[should_panic]
            fn set_invalid_lane() {
                let mut mask = Mask::<$type, 8>::splat(false);
                mask.set(8, true);
                let _ = mask;
            }

            #[test]
            #[should_panic]
            fn test_invalid_lane() {
                let mask = Mask::<$type, 8>::splat(false);
                let _ = mask.test(8);
            }

            #[test]
            fn any() {
                assert!(!Mask::<$type, 8>::splat(false).any());
                assert!(Mask::<$type, 8>::splat(true).any());
                let mut v = Mask::<$type, 8>::splat(false);
                v.set(2, true);
                assert!(v.any());
            }

            #[test]
            fn all() {
                assert!(!Mask::<$type, 8>::splat(false).all());
                assert!(Mask::<$type, 8>::splat(true).all());
                let mut v = Mask::<$type, 8>::splat(false);
                v.set(2, true);
                assert!(!v.all());