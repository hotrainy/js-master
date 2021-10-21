
#![allow(unused_imports)]
use super::MaskElement;
use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SupportedLaneCount, ToBitMask};
use core::marker::PhantomData;

/// A mask where each lane is represented by a single bit.
#[repr(transparent)]
pub struct Mask<T, const LANES: usize>(
    <LaneCount<LANES> as SupportedLaneCount>::BitMask,
    PhantomData<T>,
)
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

impl<T, const LANES: usize> PartialEq for Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}

impl<T, const LANES: usize> PartialOrd for Mask<T, LANES>
where
    T: MaskElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.as_ref().partial_cmp(other.0.as_ref())
    }