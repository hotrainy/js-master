
#![cfg_attr(feature = "as_crate", no_std)] // We are std!
#![cfg_attr(
    feature = "as_crate",
    feature(platform_intrinsics),
    feature(portable_simd)
)]
#[cfg(not(feature = "as_crate"))]
use core::simd;
#[cfg(feature = "as_crate")]
use core_simd::simd;

use simd::{LaneCount, Simd, SupportedLaneCount};

#[cfg(feature = "as_crate")]
mod experimental {
    pub trait Sealed {}
}

#[cfg(feature = "as_crate")]
use experimental as sealed;

use crate::sealed::Sealed;

// "platform intrinsics" are essentially "codegen intrinsics"
// each of these may be scalarized and lowered to a libm call
extern "platform-intrinsic" {
    // ceil
    fn simd_ceil<T>(x: T) -> T;

    // floor
    fn simd_floor<T>(x: T) -> T;

    // round
    fn simd_round<T>(x: T) -> T;

    // trunc
    fn simd_trunc<T>(x: T) -> T;

    // fsqrt
    fn simd_fsqrt<T>(x: T) -> T;

    // fma
    fn simd_fma<T>(x: T, y: T, z: T) -> T;
}

/// This trait provides a possibly-temporary implementation of float functions
/// that may, in the absence of hardware support, canonicalize to calling an
/// operating system's `math.h` dynamically-loaded library (also known as a
/// shared object). As these conditionally require runtime support, they
/// should only appear in binaries built assuming OS support: `std`.
///
/// However, there is no reason SIMD types, in general, need OS support,
/// as for many architectures an embedded binary may simply configure that
/// support itself. This means these types must be visible in `core`
/// but have these functions available in `std`.
///
/// [`f32`] and [`f64`] achieve a similar trick by using "lang items", but
/// due to compiler limitations, it is harder to implement this approach for
/// abstract data types like [`Simd`]. From that need, this trait is born.
///
/// It is possible this trait will be replaced in some manner in the future,
/// when either the compiler or its supporting runtime functions are improved.
/// For now this trait is available to permit experimentation with SIMD float
/// operations that may lack hardware support, such as `mul_add`.
pub trait StdFloat: Sealed + Sized {
    /// Fused multiply-add.  Computes `(self * a) + b` with only one rounding error,
    /// yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated `fma` CPU instruction.  However, this is not always
    /// true, and will be heavily dependent on designing algorithms with specific target