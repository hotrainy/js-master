use crate::simd::intrinsics;
use crate::simd::{LaneCount, Mask, MaskElement, Simd, SimdElement, SupportedLaneCount};

impl<T, const LANES: usize> Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Choose lanes from two vectors.
    ///
    /// For each lane in the mask, choose the corresponding lane from `true_values` if
    /// that lane mask is true, and `false_values` if that lane mask is false.
    ///
    /// # Examples
    /// ```
    /// # #![feature(