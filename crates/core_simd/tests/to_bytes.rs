#![feature(portable_simd, generic_const_exprs, adt_const_params)]
#![allow(incomplete_features)]
#![cfg(feature = "generic_const_exprs")]

use core_simd::simd::Simd;

#[test]
fn byte_convert() {
    let int = Simd::<u32, 2>::from_array([0xdeadbeef, 0x8badf00d]);
    let bytes = int.to_n