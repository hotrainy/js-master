use super::sealed::Sealed;
use crate::simd::{intrinsics, LaneCount, Mask, Simd, SimdPartialEq, SupportedLaneCount};

/// Operations on SIMD vectors of mutable pointers.
pub trait SimdMutPtr: Copy + Sealed {
    /// Vector of `usize` with the same number of lanes.
    type Usize;

    /// Vector of `isize` with the same number of lanes.
    type Isize;

    /// Vector of constant pointers to the same type.
    type ConstPtr;

    /// Mask type used for manipulating this SIMD vector type.
    type Mask;

    /// Returns `true` for each lane that is null.
    fn is_null(self) -> Self::Mask;

    /// Changes constness without changing the type.
    ///
    /// Equivalent to calling [`pointer::cast_const`] 