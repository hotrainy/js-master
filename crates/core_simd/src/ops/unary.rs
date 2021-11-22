use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};
use core::ops::{Neg, Not}; // unary ops

macro_rules! neg {
    ($(impl<const LANES: usize> Neg for Simd<$scalar:ty, LANES>)*) => {
        $(impl<const LANES: usize> Neg for Simd<$scalar, LANES>
        where
            $scalar: SimdElement,
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Output = Self;

            #[inline]
            #[must_use = "operator returns a new vector without mutating the input"]
            fn neg(self) -> Self::Output {
                // Safety: `self` is a signed vector
  