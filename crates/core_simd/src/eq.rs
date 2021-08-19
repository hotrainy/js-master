use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdConstPtr, SimdElement, SimdMutPtr, SupportedLaneCount,
};

/// Parallel `PartialEq`.
