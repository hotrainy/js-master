use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
use core::ops::{Neg, Not}; // unary ops

macro_rules! neg {
    ($(impl<const LANES: usize> Neg for Simd<$scalar:ty, LANES>)*) => {
        $(impl<const LANES: 