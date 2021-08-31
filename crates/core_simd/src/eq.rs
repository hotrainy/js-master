use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdConstPtr, SimdElement, SimdMutPtr, SupportedLaneCount,
};

/// Parallel `PartialEq`.
pub trait SimdPartialEq {
    /// The mask type returned by each comparison.
    type Mask;

    /// Test if each lane is equal to the corresponding lane in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_eq(self, other: Self) -> Self::Mask;

    /// Test if each lane is equal to the corresponding lane in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ne(self, other: Self) -> Self::Mask;
}

macro_rules! impl_number {
    { $($num