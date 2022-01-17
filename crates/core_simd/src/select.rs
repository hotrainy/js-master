use crate::simd::intrinsics;
use crate::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

impl<T, const LANES: usize> Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Choose lanes from two vectors.
    ///
    /// For each lane in the mask,