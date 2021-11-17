
//! Types and traits associated with masking lanes of vectors.
//! Types representing
#![allow(non_camel_case_types)]

#[cfg_attr(
    not(all(target_arch = "x86_64", target_feature = "avx512f")),
    path = "masks/full_masks.rs"
)]
#[cfg_attr(
    all(target_arch = "x86_64", target_feature = "avx512f"),
    path = "masks/bitmask.rs"
)]
mod mask_impl;

mod to_bitmask;
pub use to_bitmask::ToBitMask;

#[cfg(feature = "generic_const_exprs")]
pub use to_bitmask::{bitmask_len, ToBitMaskArray};

use crate::simd::{intrinsics, LaneCount, Simd, SimdElement, SimdPartialEq, SupportedLaneCount};
use core::cmp::Ordering;
use core::{fmt, mem};

mod sealed {
    use super::*;

    /// Not only does this seal the `MaskElement` trait, but these functions prevent other traits
    /// from bleeding into the parent bounds.
    ///
    /// For example, `eq` could be provided by requiring `MaskElement: PartialEq`, but that would
    /// prevent us from ever removing that bound, or from implementing `MaskElement` on
    /// non-`PartialEq` types in the future.
    pub trait Sealed {
        fn valid<const LANES: usize>(values: Simd<Self, LANES>) -> bool
        where
            LaneCount<LANES>: SupportedLaneCount,
            Self: SimdElement;

        fn eq(self, other: Self) -> bool;

        const TRUE: Self;

        const FALSE: Self;
    }
}
use sealed::Sealed;

/// Marker trait for types that may be used as SIMD mask elements.
///
/// # Safety
/// Type must be a signed integer.
pub unsafe trait MaskElement: SimdElement + Sealed {}

macro_rules! impl_element {
    { $ty:ty } => {
        impl Sealed for $ty {
            #[inline]
            fn valid<const LANES: usize>(value: Simd<Self, LANES>) -> bool
            where
                LaneCount<LANES>: SupportedLaneCount,
            {
                (value.simd_eq(Simd::splat(0 as _)) | value.simd_eq(Simd::splat(-1 as _))).all()
            }

            #[inline]
            fn eq(self, other: Self) -> bool { self == other }

            const TRUE: Self = -1;
            const FALSE: Self = 0;
        }

        // Safety: this is a valid mask element type
        unsafe impl MaskElement for $ty {}
    }
}

impl_element! { i8 }
impl_element! { i16 }
impl_element! { i32 }
impl_element! { i64 }
impl_element! { isize }

/// A SIMD vector mask for `LANES` elements of width specified by `Element`.
///
/// Masks represent boolean inclusion/exclusion on a per-lane basis.
///
/// The layout of this type is unspecified, and may change between platforms
/// and/or Rust versions, and code should not assume that it is equivalent to
/// `[T; LANES]`.
#[repr(transparent)]
pub struct Mask<T, const LANES: usize>(mask_impl::Mask<T, LANES>)
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount;

impl<T, const LANES: usize> Copy for Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
}

impl<T, const LANES: usize> Clone for Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, const LANES: usize> Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Construct a mask by setting all lanes to the given value.
    #[inline]
    pub fn splat(value: bool) -> Self {
        Self(mask_impl::Mask::splat(value))
    }

    /// Converts an array of bools to a SIMD mask.
    #[inline]
    pub fn from_array(array: [bool; LANES]) -> Self {
        // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
        //     true:    0b_0000_0001
        //     false:   0b_0000_0000
        // Thus, an array of bools is also a valid array of bytes: [u8; N]
        // This would be hypothetically valid as an "in-place" transmute,
        // but these are "dependently-sized" types, so copy elision it is!
        unsafe {
            let bytes: [u8; LANES] = mem::transmute_copy(&array);
            let bools: Simd<i8, LANES> =
                intrinsics::simd_ne(Simd::from_array(bytes), Simd::splat(0u8));
            Mask::from_int_unchecked(intrinsics::simd_cast(bools))
        }
    }

    /// Converts a SIMD mask to an array of bools.
    #[inline]
    pub fn to_array(self) -> [bool; LANES] {
        // This follows mostly the same logic as from_array.
        // SAFETY: Rust's bool has a layout of 1 byte (u8) with a value of
        //     true:    0b_0000_0001
        //     false:   0b_0000_0000
        // Thus, an array of bools is also a valid array of bytes: [u8; N]
        // Since our masks are equal to integers where all bits are set,
        // we can simply convert them to i8s, and then bitand them by the