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

    from_transmute! { unsafe u8x8 => uint8x8_t }
    from_transmute! { unsafe u8x16 => uint8x16_t }
    from_transmute! { unsafe i8x8 => int8x8_t }
    from_transmute! { unsafe i8x16 => int8x16_t }
    from_transmute! { unsafe u8x8 => poly8x8_t }
    from_transmute! { unsafe u8x16 => poly8x16_t }

    from_transmute! { unsafe u16x4 => uint16x4_t }
    from_transmute! { unsafe u16x8 => uint16x8_t }
    from_transmute! { unsafe i16x4 => int16x4_t }
    from_transmute! { unsafe i16x8 => int16x8_t }
    from_transmute! { unsafe u16x4 => poly16x4_t }
    from_transmute! { unsafe u16x8 => poly16x8_t }

    from_transmute! { unsafe u32x2 => uint32x2_t }
    from_transmute! { unsafe u32x4 => uint32x4_t }
    from_transmute! { unsafe i32x2 => int32x2_t }
    from_transmute! { unsafe i32x4 => int3