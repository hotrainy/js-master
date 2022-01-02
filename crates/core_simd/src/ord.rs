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
pub trait SimdOrd: SimdPartialOrd {
    /// Returns the lane-wise maximum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_max(self, other: Self) -> Self;

    /// Returns the lane-wise minimum with `other`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_min(self, other: Self) -> Self;

    /// Restrict each lane to a certain interval.
    ///
    /// For each lane, returns `max` if `self` is greater than `max`, and `min` if `self` is
    /// less than `min`. Otherwise returns `self`.
    ///
    /// # Panics
    ///
    /// Panics if `min > max` on any lane.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_clamp(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_integer {
    { $($integer:ty),* } => {
        $(
        impl<const LANES: usize> SimdPartialOrd for Simd<$integer, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline]
            fn simd_lt(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_lt(self, other)) }
            }

            #[inline]
            fn simd_le(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_le(self, other)) }
            }

            #[inline]
            fn simd_gt(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_gt(self, other)) }
            }

            #[inline]
            fn simd_ge(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_ge(self, other)) }
            }
        }

        impl<const LANES: usize> SimdOrd for Simd<$integer, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline]
            fn simd_max(self, other: Self) -> Self {
                self.simd_lt(other).select(other, self)
            }

            #[inline]
            fn simd_min(self, other: Self) -> Self {
                self.simd_gt(other).select(other, self)
            }

            #[inline]
            fn simd_clamp(self, min: Self, max: Self) -> Self {
                assert!(
                    min.simd_le(max).all(),
                    "each lane in `min` must be less than or equal to the corresponding lane in `max`",
                );
                self.simd_max(min).simd_min(max)
            }
        }
        )*
    }
}

impl_integer! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

macro_rules! impl_float {
    { $($float:ty),* } => {
        $(
        impl<const LANES: usize> SimdPartialOrd for Simd<$float, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline]
            fn simd_lt(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_lt(self, other)) }
            }

            #[inline]
            fn simd_le(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_le(self, other)) }
            }

            #[inline]
            fn simd_gt(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_gt(self, other)) }
            }

            #[inline]
            fn simd_ge(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Mask::from_int_unchecked(intrinsics::simd_ge(self, other)) }
            }
        }
        )*
    }
}

impl_float! { f32, f64 }

macro_rules! impl_mask {
    { $($integer:ty),* } => {
        $(
        impl<const LANES: usize> SimdPartialOrd for Mask<$integer, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            #[inline]
            fn simd_lt(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_int_unchecked(intrinsics::simd_lt(self.to_int(), other.to_int())) }
            }

            #[inline]
            fn simd_le(self, other: Self) -> Self::Mask {
                // Safety: `self` is a vector, and the result of the comparison
                // is always a valid mask.
                unsafe { Self::from_int_unchecked(intrinsics::simd_le(self.to_int(), other.to_int())) }
            }

            #[inline]
            fn simd_gt(self