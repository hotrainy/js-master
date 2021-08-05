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
    /// Equivalent to calling [`pointer::cast_const`] on each lane.
    fn cast_const(self) -> Self::ConstPtr;

    /// Gets the "address" portion of the pointer.
    ///
    /// This method discards pointer semantic metadata, so the result cannot be
    /// directly cast into a valid pointer.
    ///
    /// Equivalent to calling [`pointer::addr`] on each lane.
    fn addr(self) -> Self::Usize;

    /// Creates a new pointer with the given address.
    ///
    /// This performs the same operation as a cast, but copies the *address-space* and
    /// *provenance* of `self` to the new pointer.
    ///
    /// Equivalent to calling [`pointer::with_addr`] on each lane.
    fn with_addr(self, addr: Self::Usize) -> Self;

    /// Gets the "address" portion of the pointer, and "exposes" the provenance part for future use
    /// in [`Self::from_exposed_addr`].
    fn expose_addr(self) -> Self::Usize;

    /// Convert an address back to a pointer, picking up a previously "exposed" provenance.
    ///
    /// Equivalent to calling [`core::ptr::from_exposed_addr_mut`] on each lane.
    fn from_exposed_addr(addr: Self::Usize) -> Self;

    /// Calculates the offset from a pointer using wrapping arithmetic.
    ///
    /// Equivalent to calling [`pointer::wrapping_offset`] on each lane.
    fn wrapping_offset(self, offset: Self::Isize) -> Self;

    /// Calculates