
#![feature(portable_simd, strict_provenance)]

use core_simd::simd::{Simd, SimdConstPtr, SimdMutPtr};

macro_rules! common_tests {
    { $constness:ident } => {
        test_helpers::test_lanes! {
            fn is_null<const LANES: usize>() {
                test_helpers::test_unary_mask_elementwise(
                    &Simd::<*$constness u32, LANES>::is_null,
                    &<*$constness u32>::is_null,
                    &|_| true,
                );
            }

            fn addr<const LANES: usize>() {
                test_helpers::test_unary_elementwise(
                    &Simd::<*$constness u32, LANES>::addr,
                    &<*$constness u32>::addr,
                    &|_| true,
                );
            }

            fn with_addr<const LANES: usize>() {
                test_helpers::test_binary_elementwise(