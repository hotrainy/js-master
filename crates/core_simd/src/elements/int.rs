
use super::sealed::Sealed;
use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdElement, SimdPartialOrd, SupportedLaneCount,
};

/// Operations on SIMD vectors of signed integers.
pub trait SimdInt: Copy + Sealed {
    /// Mask type used for manipulating this SIMD vector type.
    type Mask;

    /// Scalar type contained by this SIMD vector type.
    type Scalar;

    /// Lanewise saturating add.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{Simd, SimdInt};
    /// use core::i32::{MIN, MAX};
    /// let x = Simd::from_array([MIN, 0, 1, MAX]);
    /// let max = Simd::splat(MAX);
    /// let unsat = x + max;
    /// let sat = x.saturating_add(max);
    /// assert_eq!(unsat, Simd::from_array([-1, MAX, MIN, -2]));
    /// assert_eq!(sat, Simd::from_array([-1, MAX, MAX, MAX]));
    /// ```
    fn saturating_add(self, second: Self) -> Self;

    /// Lanewise saturating subtract.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{Simd, SimdInt};
    /// use core::i32::{MIN, MAX};
    /// let x = Simd::from_array([MIN, -2, -1, MAX]);
    /// let max = Simd::splat(MAX);
    /// let unsat = x - max;
    /// let sat = x.saturating_sub(max);
    /// assert_eq!(unsat, Simd::from_array([1, MAX, MIN, 0]));
    /// assert_eq!(sat, Simd::from_array([MIN, MIN, MIN, 0]));
    fn saturating_sub(self, second: Self) -> Self;

    /// Lanewise absolute value, implemented in Rust.
    /// Every lane becomes its absolute value.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{Simd, SimdInt};
    /// use core::i32::{MIN, MAX};
    /// let xs = Simd::from_array([MIN, MIN +1, -5, 0]);
    /// assert_eq!(xs.abs(), Simd::from_array([MIN, MAX, 5, 0]));
    /// ```
    fn abs(self) -> Self;

    /// Lanewise saturating absolute value, implemented in Rust.
    /// As abs(), except the MIN value becomes MAX instead of itself.
    ///