
use crate::simd::{
    intrinsics, LaneCount, Mask, MaskElement, SimdCast, SimdCastPtr, SimdConstPtr, SimdMutPtr,
    SimdPartialOrd, SupportedLaneCount, Swizzle,
};

/// A SIMD vector of `LANES` elements of type `T`. `Simd<T, N>` has the same shape as [`[T; N]`](array), but operates like `T`.
///
/// Two vectors of the same type and length will, by convention, support the operators (+, *, etc.) that `T` does.
/// These take the lanes at each index on the left-hand side and right-hand side, perform the operation,
/// and return the result in the same lane in a vector of equal size. For a given operator, this is equivalent to zipping
/// the two arrays together and mapping the operator over each lane.
///
/// ```rust
/// # #![feature(array_zip, portable_simd)]
/// # use core::simd::{Simd};
/// let a0: [i32; 4] = [-2, 0, 2, 4];
/// let a1 = [10, 9, 8, 7];
/// let zm_add = a0.zip(a1).map(|(lhs, rhs)| lhs + rhs);
/// let zm_mul = a0.zip(a1).map(|(lhs, rhs)| lhs * rhs);
///
/// // `Simd<T, N>` implements `From<[T; N]>
/// let (v0, v1) = (Simd::from(a0), Simd::from(a1));
/// // Which means arrays implement `Into<Simd<T, N>>`.
/// assert_eq!(v0 + v1, zm_add.into());
/// assert_eq!(v0 * v1, zm_mul.into());
/// ```
///
/// `Simd` with integers has the quirk that these operations are also inherently wrapping, as if `T` was [`Wrapping<T>`].
/// Thus, `Simd` does not implement `wrapping_add`, because that is the default behavior.
/// This means there is no warning on overflows, even in "debug" builds.
/// For most applications where `Simd` is appropriate, it is "not a bug" to wrap,
/// and even "debug builds" are unlikely to tolerate the loss of performance.
/// You may want to consider using explicitly checked arithmetic if such is required.
/// Division by zero still causes a panic, so you may want to consider using floating point numbers if that is unacceptable.
///
/// [`Wrapping<T>`]: core::num::Wrapping
///
/// # Layout
/// `Simd<T, N>` has a layout similar to `[T; N]` (identical "shapes"), but with a greater alignment.
/// `[T; N]` is aligned to `T`, but `Simd<T, N>` will have an alignment based on both `T` and `N`.
/// It is thus sound to [`transmute`] `Simd<T, N>` to `[T; N]`, and will typically optimize to zero cost,
/// but the reverse transmutation is more likely to require a copy the compiler cannot simply elide.
///
/// # ABI "Features"
/// Due to Rust's safety guarantees, `Simd<T, N>` is currently passed to and from functions via memory, not SIMD registers,
/// except as an optimization. `#[inline]` hints are recommended on functions that accept `Simd<T, N>` or return it.
/// The need for this may be corrected in the future.
///
/// # Safe SIMD with Unsafe Rust
///
/// Operations with `Simd` are typically safe, but there are many reasons to want to combine SIMD with `unsafe` code.
/// Care must be taken to respect differences between `Simd` and other types it may be transformed into or derived from.
/// In particular, the layout of `Simd<T, N>` may be similar to `[T; N]`, and may allow some transmutations,
/// but references to `[T; N]` are not interchangeable with those to `Simd<T, N>`.
/// Thus, when using `unsafe` Rust to read and write `Simd<T, N>` through [raw pointers], it is a good idea to first try with
/// [`read_unaligned`] and [`write_unaligned`]. This is because:
/// - [`read`] and [`write`] require full alignment (in this case, `Simd<T, N>`'s alignment)
/// - the likely source for reading or destination for writing `Simd<T, N>` is [`[T]`](slice) and similar types, aligned to `T`
/// - combining these actions would violate the `unsafe` contract and explode the program into a puff of **undefined behavior**
/// - the compiler can implicitly adjust layouts to make unaligned reads or writes fully aligned if it sees the optimization
/// - most contemporary processors suffer no performance penalty for "unaligned" reads and writes that are aligned at runtime
///
/// By imposing less obligations, unaligned functions are less likely to make the program unsound,
/// and may be just as fast as stricter alternatives.
/// When trying to guarantee alignment, [`[T]::as_simd`][as_simd] is an option for converting `[T]` to `[Simd<T, N>]`,
/// and allows soundly operating on an aligned SIMD body, but it may cost more time when handling the scalar head and tail.
/// If these are not sufficient, then it is most ideal to design data structures to be already aligned
/// to the `Simd<T, N>` you wish to use before using `unsafe` Rust to read or write.
/// More conventional ways to compensate for these facts, like materializing `Simd` to or from an array first,
/// are handled by safe methods like [`Simd::from_array`] and [`Simd::from_slice`].
///
/// [`transmute`]: core::mem::transmute
/// [raw pointers]: pointer
/// [`read_unaligned`]: pointer::read_unaligned
/// [`write_unaligned`]: pointer::write_unaligned
/// [`read`]: pointer::read
/// [`write`]: pointer::write
/// [as_simd]: slice::as_simd
#[repr(simd)]
pub struct Simd<T, const LANES: usize>([T; LANES])
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount;

impl<T, const LANES: usize> Simd<T, LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
    T: SimdElement,
{
    /// Number of lanes in this vector.
    pub const LANES: usize = LANES;

    /// Returns the number of lanes in this SIMD vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::u32x4;
    /// let v = u32x4::splat(0);
    /// assert_eq!(v.lanes(), 4);
    /// ```
    pub const fn lanes(&self) -> usize {
        LANES
    }

    /// Constructs a new SIMD vector with all lanes set to the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::u32x4;
    /// let v = u32x4::splat(8);
    /// assert_eq!(v.as_array(), &[8, 8, 8, 8]);
    /// ```
    pub fn splat(value: T) -> Self {
        // This is preferred over `[value; LANES]`, since it's explicitly a splat:
        // https://github.com/rust-lang/rust/issues/97804
        struct Splat;
        impl<const LANES: usize> Swizzle<1, LANES> for Splat {
            const INDEX: [usize; LANES] = [0; LANES];
        }
        Splat::swizzle(Simd::<T, 1>::from([value]))
    }

    /// Returns an array reference containing the entire SIMD vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::{Simd, u64x4};
    /// let v: u64x4 = Simd::from_array([0, 1, 2, 3]);
    /// assert_eq!(v.as_array(), &[0, 1, 2, 3]);
    /// ```
    pub const fn as_array(&self) -> &[T; LANES] {
        &self.0
    }

    /// Returns a mutable array reference containing the entire SIMD vector.
    pub fn as_mut_array(&mut self) -> &mut [T; LANES] {
        &mut self.0
    }

    /// Converts an array to a SIMD vector.
    pub const fn from_array(array: [T; LANES]) -> Self {
        Self(array)
    }

    /// Converts a SIMD vector to an array.
    pub const fn to_array(self) -> [T; LANES] {
        self.0
    }

    /// Converts a slice to a SIMD vector containing `slice[..LANES]`.
    ///
    /// # Panics
    ///
    /// Panics if the slice's length is less than the vector's `Simd::LANES`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::u32x4;
    /// let source = vec![1, 2, 3, 4, 5, 6];
    /// let v = u32x4::from_slice(&source);
    /// assert_eq!(v.as_array(), &[1, 2, 3, 4]);
    /// ```
    #[must_use]
    pub const fn from_slice(slice: &[T]) -> Self {
        assert!(
            slice.len() >= LANES,
            "slice length must be at least the number of lanes"
        );
        // Safety:
        // - We've checked the length is sufficient.
        // - `T` and `Simd<T, N>` are Copy types.
        unsafe { slice.as_ptr().cast::<Self>().read_unaligned() }
    }

    /// Performs lanewise conversion of a SIMD vector's elements to another SIMD-valid type.
    ///
    /// This follows the semantics of Rust's `as` conversion for casting
    /// integers to unsigned integers (interpreting as the other type, so `-1` to `MAX`),
    /// and from floats to integers (truncating, or saturating at the limits) for each lane,
    /// or vice versa.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::Simd;
    /// let floats: Simd<f32, 4> = Simd::from_array([1.9, -4.5, f32::INFINITY, f32::NAN]);
    /// let ints = floats.cast::<i32>();
    /// assert_eq!(ints, Simd::from_array([1, -4, i32::MAX, 0]));
    ///
    /// // Formally equivalent, but `Simd::cast` can optimize better.
    /// assert_eq!(ints, Simd::from_array(floats.to_array().map(|x| x as i32)));
    ///
    /// // The float conversion does not round-trip.
    /// let floats_again = ints.cast();
    /// assert_ne!(floats, floats_again);
    /// assert_eq!(floats_again, Simd::from_array([1.0, -4.0, 2147483647.0, 0.0]));
    /// ```
    #[must_use]
    #[inline]
    #[cfg(not(bootstrap))]
    pub fn cast<U: SimdCast>(self) -> Simd<U, LANES>
    where
        T: SimdCast,
    {
        // Safety: supported types are guaranteed by SimdCast
        unsafe { intrinsics::simd_as(self) }
    }

    /// Lanewise casts pointers to another pointer type.
    #[must_use]
    #[inline]
    pub fn cast_ptr<U>(self) -> Simd<U, LANES>
    where
        T: SimdCastPtr<U>,
        U: SimdElement,
    {
        // Safety: supported types are guaranteed by SimdCastPtr
        unsafe { intrinsics::simd_cast_ptr(self) }
    }

    /// Rounds toward zero and converts to the same-width integer type, assuming that
    /// the value is finite and fits in that type.
    ///
    /// # Safety
    /// The value must:
    ///
    /// * Not be NaN
    /// * Not be infinite
    /// * Be representable in the return type, after truncating off its fractional part
    ///
    /// If these requirements are infeasible or costly, consider using the safe function [cast],
    /// which saturates on conversion.
    ///
    /// [cast]: Simd::cast
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn to_int_unchecked<I>(self) -> Simd<I, LANES>
    where
        T: core::convert::FloatToInt<I> + SimdCast,
        I: SimdCast,
    {
        // Safety: supported types are guaranteed by SimdCast, the caller is responsible for the extra invariants
        unsafe { intrinsics::simd_cast(self) }
    }

    /// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
    /// If an index is out-of-bounds, the lane is instead selected from the `or` vector.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::Simd;
    /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
    /// let idxs = Simd::from_array([9, 3, 0, 5]);
    /// let alt = Simd::from_array([-5, -4, -3, -2]);
    ///
    /// let result = Simd::gather_or(&vec, idxs, alt); // Note the lane that is out-of-bounds.
    /// assert_eq!(result, Simd::from_array([-5, 13, 10, 15]));
    /// ```
    #[must_use]
    #[inline]
    pub fn gather_or(slice: &[T], idxs: Simd<usize, LANES>, or: Self) -> Self {
        Self::gather_select(slice, Mask::splat(true), idxs, or)
    }

    /// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
    /// If an index is out-of-bounds, the lane is set to the default value for the type.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::Simd;
    /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
    /// let idxs = Simd::from_array([9, 3, 0, 5]);
    ///
    /// let result = Simd::gather_or_default(&vec, idxs); // Note the lane that is out-of-bounds.
    /// assert_eq!(result, Simd::from_array([0, 13, 10, 15]));
    /// ```
    #[must_use]
    #[inline]
    pub fn gather_or_default(slice: &[T], idxs: Simd<usize, LANES>) -> Self
    where
        T: Default,
    {
        Self::gather_or(slice, idxs, Self::splat(T::default()))
    }

    /// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
    /// The mask `enable`s all `true` lanes and disables all `false` lanes.
    /// If an index is disabled or is out-of-bounds, the lane is selected from the `or` vector.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::{Simd, Mask};
    /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
    /// let idxs = Simd::from_array([9, 3, 0, 5]);
    /// let alt = Simd::from_array([-5, -4, -3, -2]);
    /// let enable = Mask::from_array([true, true, true, false]); // Note the mask of the last lane.
    ///
    /// let result = Simd::gather_select(&vec, enable, idxs, alt); // Note the lane that is out-of-bounds.
    /// assert_eq!(result, Simd::from_array([-5, 13, 10, -2]));
    /// ```
    #[must_use]
    #[inline]
    pub fn gather_select(
        slice: &[T],
        enable: Mask<isize, LANES>,
        idxs: Simd<usize, LANES>,
        or: Self,
    ) -> Self {
        let enable: Mask<isize, LANES> = enable & idxs.simd_lt(Simd::splat(slice.len()));
        // Safety: We have masked-off out-of-bounds lanes.
        unsafe { Self::gather_select_unchecked(slice, enable, idxs, or) }
    }

    /// Reads from potentially discontiguous indices in `slice` to construct a SIMD vector.
    /// The mask `enable`s all `true` lanes and disables all `false` lanes.
    /// If an index is disabled, the lane is selected from the `or` vector.
    ///
    /// # Safety
    ///
    /// Calling this function with an `enable`d out-of-bounds index is *[undefined behavior]*
    /// even if the resulting value is not used.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{Simd, SimdPartialOrd, Mask};
    /// let vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
    /// let idxs = Simd::from_array([9, 3, 0, 5]);
    /// let alt = Simd::from_array([-5, -4, -3, -2]);
    /// let enable = Mask::from_array([true, true, true, false]); // Note the final mask lane.
    /// // If this mask was used to gather, it would be unsound. Let's fix that.
    /// let enable = enable & idxs.simd_lt(Simd::splat(vec.len()));
    ///
    /// // We have masked the OOB lane, so it's safe to gather now.
    /// let result = unsafe { Simd::gather_select_unchecked(&vec, enable, idxs, alt) };
    /// assert_eq!(result, Simd::from_array([-5, 13, 10, -2]));
    /// ```
    /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    #[must_use]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn gather_select_unchecked(
        slice: &[T],
        enable: Mask<isize, LANES>,
        idxs: Simd<usize, LANES>,
        or: Self,
    ) -> Self {
        let base_ptr = Simd::<*const T, LANES>::splat(slice.as_ptr());
        // Ferris forgive me, I have done pointer arithmetic here.
        let ptrs = base_ptr.wrapping_add(idxs);
        // Safety: The caller is responsible for determining the indices are okay to read
        unsafe { Self::gather_select_ptr(ptrs, enable, or) }
    }

    /// Read pointers elementwise into a SIMD vector.
    ///
    /// # Safety
    ///
    /// Each read must satisfy the same conditions as [`core::ptr::read`].
    ///
    /// # Example
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{Simd, SimdConstPtr};
    /// let values = [6, 2, 4, 9];
    /// let offsets = Simd::from_array([1, 0, 0, 3]);
    /// let source = Simd::splat(values.as_ptr()).wrapping_add(offsets);
    /// let gathered = unsafe { Simd::gather_ptr(source) };
    /// assert_eq!(gathered, Simd::from_array([2, 6, 6, 9]));
    /// ```
    #[must_use]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn gather_ptr(source: Simd<*const T, LANES>) -> Self
    where
        T: Default,
    {
        // TODO: add an intrinsic that doesn't use a passthru vector, and remove the T: Default bound
        // Safety: The caller is responsible for upholding all invariants
        unsafe { Self::gather_select_ptr(source, Mask::splat(true), Self::default()) }
    }

    /// Conditionally read pointers elementwise into a SIMD vector.
    /// The mask `enable`s all `true` lanes and disables all `false` lanes.
    /// If a lane is disabled, the lane is selected from the `or` vector and no read is performed.
    ///
    /// # Safety
    ///
    /// Enabled lanes must satisfy the same conditions as [`core::ptr::read`].
    ///
    /// # Example
    /// ```
    /// # #![feature(portable_simd)]
    /// # #[cfg(feature = "as_crate")] use core_simd::simd;
    /// # #[cfg(not(feature = "as_crate"))] use core::simd;
    /// # use simd::{Mask, Simd, SimdConstPtr};
    /// let values = [6, 2, 4, 9];
    /// let enable = Mask::from_array([true, true, false, true]);
    /// let offsets = Simd::from_array([1, 0, 0, 3]);
    /// let source = Simd::splat(values.as_ptr()).wrapping_add(offsets);
    /// let gathered = unsafe { Simd::gather_select_ptr(source, enable, Simd::splat(0)) };
    /// assert_eq!(gathered, Simd::from_array([2, 6, 0, 9]));
    /// ```
    #[must_use]
    #[inline]
    #[cfg_attr(miri, track_caller)] // even without panics, this helps for Miri backtraces
    pub unsafe fn gather_select_ptr(
        source: Simd<*const T, LANES>,
        enable: Mask<isize, LANES>,
        or: Self,
    ) -> Self {
        // Safety: The caller is responsible for upholding all invariants
        unsafe { intrinsics::simd_gather(or, source, enable.to_int()) }
    }

    /// Writes the values in a SIMD vector to potentially discontiguous indices in `slice`.
    /// If two lanes in the scattered vector would write to the same index
    /// only the last lane is guaranteed to actually be written.
    ///
    /// # Examples
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::Simd;
    /// let mut vec: Vec<i32> = vec![10, 11, 12, 13, 14, 15, 16, 17, 18];
    /// let idxs = Simd::from_array([9, 3, 0, 0]);
    /// let vals = Simd::from_array([-27, 82, -41, 124]);
    ///
    /// vals.scatter(&mut vec, idxs); // index 0 receives two writes.
    /// assert_eq!(vec, vec![124, 11, 12, 82, 14, 15, 16, 17, 18]);
    /// ```
    #[inline]