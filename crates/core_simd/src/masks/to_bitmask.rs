use super::{mask_impl, Mask, MaskElement};
use crate::simd::{LaneCount, SupportedLaneCount};

mod sealed {
    pub trait Sealed {}
}
pub use sealed::Sealed;

impl<T, const LANES: usize> Sealed for Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
}

/// Converts masks to and from integer bitmasks.
///
/// Each bit of the bitmask corresponds to a mask lane, starting with the LSB.
pub trait ToBitMask: Sealed {
 