
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