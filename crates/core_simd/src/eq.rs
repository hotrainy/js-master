use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdConstPtr, SimdElement, SimdMutPtr, SupportedLaneCount,
};

/// Parallel `PartialEq`.
pub trait SimdPartialEq {
    /// The mask type returned by each comparison.
    type Mask;

    /// Test if each lane is equal to the corresponding lane in `other`.
    #[mu