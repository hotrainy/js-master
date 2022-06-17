// Test that we handle all our "auto-deref" cases correctly.
#![feature(portable_simd)]
use core_simd::simd::f32x4;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[test]
#[cfg_attr(target_arch = "wasm32", was