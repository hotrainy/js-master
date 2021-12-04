use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdConstPtr, SimdMutPtr, SimdPartialEq, SupportedLaneCount,
};

/// Parallel `PartialOrd`.
pub trait SimdPartialOrd: SimdPartialEq {
    /// Test if each lane is less than the corresponding lane in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_lt(self, other: Self) -> Self::Mask;

    /// Test if each lane is less than or equal to the corresponding lane in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_le(self, other: Self) -> Self::Mask;

    /// Test if each lane is greater than the corresponding lane in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_gt(self, other: Self) -> Self::Mask;

    /// Test if each lane is greater than or equal to the corresponding lane in `other`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn simd_ge(self, other: Self) -> Self::Mask;
}

/// Parallel `Ord`.
pub trait SimdOrd: SimdPartialOrd