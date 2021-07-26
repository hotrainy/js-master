
use super::sealed::Sealed;
use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdElement, SimdPartialEq, SimdPartialOrd,
    SupportedLaneCount,
};

/// Operations on SIMD vectors of floats.
pub trait SimdFloat: Copy + Sealed {
    /// Mask type used for manipulating this SIMD vector type.
    type Mask;

    /// Scalar type contained by this SIMD vector type.
    type Scalar;

    /// Bit representation of this SIMD vector type.
    type Bits;

    /// Raw transmutation to an unsigned integer vector type with the
    /// same size and number of lanes.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn to_bits(self) -> Self::Bits;

    /// Raw transmutation from an unsigned integer vector type with the
    /// same size and number of lanes.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn from_bits(bits: Self::Bits) -> Self;

    /// Produces a vector where every lane has the absolute value of the
    /// equivalently-indexed lane in `self`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn abs(self) -> Self;