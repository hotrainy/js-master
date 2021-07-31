
use super::sealed::Sealed;
use crate::simd::{
    intrinsics, LaneCount, Mask, Simd, SimdElement, SimdPartialEq, SimdPartialOrd,
    SupportedLaneCount,
};

/// Operations on SIMD vectors of floats.
pub trait SimdFloat: Copy + Sealed {
    /// Mask type used for manipulating this SIMD vector type.
    type Mask;

    /// Scalar type contained by this SIMD vector type.
    type Scalar;

    /// Bit representation of this SIMD vector type.
    type Bits;

    /// Raw transmutation to an unsigned integer vector type with the
    /// same size and number of lanes.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn to_bits(self) -> Self::Bits;

    /// Raw transmutation from an unsigned integer vector type with the
    /// same size and number of lanes.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn from_bits(bits: Self::Bits) -> Self;

    /// Produces a vector where every lane has the absolute value of the
    /// equivalently-indexed lane in `self`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn abs(self) -> Self;

    /// Takes the reciprocal (inverse) of each lane, `1/x`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn recip(self) -> Self;

    /// Converts each lane from radians to degrees.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn to_degrees(self) -> Self;

    /// Converts each lane from degrees to radians.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn to_radians(self) -> Self;

    /// Returns true for each lane if it has a positive sign, including
    /// `+0.0`, `NaN`s with positive sign bit and positive infinity.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_sign_positive(self) -> Self::Mask;

    /// Returns true for each lane if it has a negative sign, including
    /// `-0.0`, `NaN`s with negative sign bit and negative infinity.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_sign_negative(self) -> Self::Mask;

    /// Returns true for each lane if its value is `NaN`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_nan(self) -> Self::Mask;

    /// Returns true for each lane if its value is positive infinity or negative infinity.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_infinite(self) -> Self::Mask;

    /// Returns true for each lane if its value is neither infinite nor `NaN`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_finite(self) -> Self::Mask;

    /// Returns true for each lane if its value is subnormal.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_subnormal(self) -> Self::Mask;

    /// Returns true for each lane if its value is neither zero, infinite,
    /// subnormal, nor `NaN`.
    #[must_use = "method returns a new mask and does not mutate the original value"]
    fn is_normal(self) -> Self::Mask;

    /// Replaces each lane with a number that represents its sign.
    ///
    /// * `1.0` if the number is positive, `+0.0`, or `INFINITY`
    /// * `-1.0` if the number is negative, `-0.0`, or `NEG_INFINITY`
    /// * `NAN` if the number is `NAN`
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn signum(self) -> Self;

    /// Returns each lane with the magnitude of `self` and the sign of `sign`.
    ///
    /// For any lane containing a `NAN`, a `NAN` with the sign of `sign` is returned.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn copysign(self, sign: Self) -> Self;

    /// Returns the minimum of each lane.
    ///
    /// If one of the values is `NAN`, then the other value is returned.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_min(self, other: Self) -> Self;

    /// Returns the maximum of each lane.
    ///
    /// If one of the values is `NAN`, then the other value is returned.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_max(self, other: Self) -> Self;

    /// Restrict each lane to a certain interval unless it is NaN.
    ///
    /// For each lane in `self`, returns the corresponding lane in `max` if the lane is
    /// greater than `max`, and the corresponding lane in `min` if the lane is less
    /// than `min`.  Otherwise returns the lane in `self`.
    #[must_use = "method returns a new vector and does not mutate the original value"]
    fn simd_clamp(self, min: Self, max: Self) -> Self;

    /// Returns the sum of the lanes of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{f32x2, SimdFloat};
    /// let v = f32x2::from_array([1., 2.]);
    /// assert_eq!(v.reduce_sum(), 3.);
    /// ```
    fn reduce_sum(self) -> Self::Scalar;

    /// Reducing multiply.  Returns the product of the lanes of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{f32x2, SimdFloat};
    /// let v = f32x2::from_array([3., 4.]);
    /// assert_eq!(v.reduce_product(), 12.);
    /// ```
    fn reduce_product(self) -> Self::Scalar;

    /// Returns the maximum lane in the vector.
    ///
    /// Returns values based on equality, so a vector containing both `0.` and `-0.` may
    /// return either.
    ///
    /// This function will not return `NaN` unless all lanes are `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{f32x2, SimdFloat};
    /// let v = f32x2::from_array([1., 2.]);
    /// assert_eq!(v.reduce_max(), 2.);
    ///
    /// // NaN values are skipped...
    /// let v = f32x2::from_array([1., f32::NAN]);
    /// assert_eq!(v.reduce_max(), 1.);
    ///
    /// // ...unless all values are NaN
    /// let v = f32x2::from_array([f32::NAN, f32::NAN]);
    /// assert!(v.reduce_max().is_nan());
    /// ```
    fn reduce_max(self) -> Self::Scalar;

    /// Returns the minimum lane in the vector.
    ///
    /// Returns values based on equality, so a vector containing both `0.` and `-0.` may
    /// return either.
    ///
    /// This function will not return `NaN` unless all lanes are `NaN`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{f32x2, SimdFloat};
    /// let v = f32x2::from_array([3., 7.]);
    /// assert_eq!(v.reduce_min(), 3.);
    ///
    /// // NaN values are skipped...
    /// let v = f32x2::from_array([1., f32::NAN]);
    /// assert_eq!(v.reduce_min(), 1.);
    ///
    /// // ...unless all values are NaN
    /// let v = f32x2::from_array([f32::NAN, f32::NAN]);
    /// assert!(v.reduce_min().is_nan());
    /// ```
    fn reduce_min(self) -> Self::Scalar;
}

macro_rules! impl_trait {
    { $($ty:ty { bits: $bits_ty:ty, mask: $mask_ty:ty }),* } => {
        $(
        impl<const LANES: usize> Sealed for Simd<$ty, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
        }

        impl<const LANES: usize> SimdFloat for Simd<$ty, LANES>
        where
            LaneCount<LANES>: SupportedLaneCount,
        {
            type Mask = Mask<<$mask_ty as SimdElement>::Mask, LANES>;
            type Scalar = $ty;
            type Bits = Simd<$bits_ty, LANES>;

            #[inline]
            fn to_bits(self) -> Simd<$bits_ty, LANES> {
                assert_eq!(core::mem::size_of::<Self>(), core::mem::size_of::<Self::Bits>());
                // Safety: transmuting between vector types is safe
                unsafe { core::mem::transmute_copy(&self) }
            }

            #[inline]
            fn from_bits(bits: Simd<$bits_ty, LANES>) -> Self {
                assert_eq!(core::mem::size_of::<Self>(), core::mem::size_of::<Self::Bits>());
                // Safety: transmuting between vector types is safe
                unsafe { core::mem::transmute_copy(&bits) }
            }

            #[inline]
            fn abs(self) -> Self {