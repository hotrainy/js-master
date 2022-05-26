#![allow(unused)]
use crate::simd::*;

#[cfg(target_arch = "arm")]
use core::arch::arm::*;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "arm", target_feature = "v7"),
))]
mod neon {
    use super::*;

    from_transmute! { unsafe f32x2 => float32x2_t }
    from_transmute! { unsafe f32x4 => float32x4_t }

    from_transmute! { unsafe u8