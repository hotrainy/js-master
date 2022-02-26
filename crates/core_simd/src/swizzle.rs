use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

/// Constructs a new SIMD vector by copying elements from selected lanes in other vectors.
///
/// When swizzling one vector, lanes are selected by a `const` array of `usize`,
/// like [`Swizzle`].
///
/// When swizzling two vectors, lanes are selected by a `const` array of [`Which`],
/// like [`Swizzle2`].
///
/// # Examples
///
/// With a single SIMD vector, the const array specifies lane indices in that vector:
/// ```
/// # #![feature(portable_simd)]
/// # use core::simd::{u32x2, u32x4, simd_swizzle};
/// let v = u32x4::from_array([10, 11, 12, 13]);
///
/// // Keeping the same size
/// let r: u32x4 = simd_swizzle!(v, [3, 0, 1, 2]);
/// assert_eq!(r.to_array(), [13, 10, 11, 12]);
///
/// // Changing the number of lanes
/// let r: u32x2 = simd_swizzle!(v, [3, 1]);
/// assert_eq!(r.to_array(), [13, 11]);
/// ```
///
/// With two input SIMD vectors, the const array uses `Which` to specify the source of each index:
/// ```
/// # #![feature(portable_simd)]
/// # use core::simd::{u32x2, u32x4, simd_swizzle, Which};
/// use Which::{First, Second};
/// let a = u32x4::from_array([0, 1, 2, 3]);
/// let b = u32x4::from_array([4, 5, 6, 7]);
///
/// // Keeping the same size
/// let r: u32x4 = simd_swizzle!(a, b, [First(0), First(1), Second(2), Second(3)]);
/// assert_eq!(r.to_array(), [0, 1, 6, 7]);
///
/// // Changing the number of lanes
/// let r: u32x2 = simd_swizzle!(a, b, [First(0), Second(0)]);
/// assert_eq!(r.to_array(), [0, 4]);
/// ```
#[allow(unused_macros)]
pub macro simd_swizzle {
    (
        $vector:expr, $index:expr $(,)?
    ) => {
        {
            use $crate::simd::Swizzle;
            struct Impl;
            impl<const LANES: usize> Swizzle<LANES, {$index.len()}> for Impl {
                const INDEX: [usize; {$index.len()}] = $index;
            }
            Impl::swizzle($vector)
        }
    },
    (
        $first:expr, $second:expr, $index:expr $(,)?
    ) => {
        {
            use $crate::simd::{Which, Swizzle2};
            struct Impl;
            impl<const LANES: usize> Swizzle2<LANES, {$index.len()}> for Impl {
                const INDEX: [Which; {$index.len()}] = $index;
            }
            Impl::swizzle2($first, $second)
        }
    }
}

/// Specifies a lane index into one of two SIMD vectors.
///
/// This is an input type for [Swizzle2] and helper macros like [simd_swizzle].
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Which {
    /// Index of a lane in the first input SIMD vector.
    First(usize),
    /// Index of a lane in the second input SIMD vector.
    Second(usize),
}

/// Create a vector from the elements of another vector.
pub trait Swizzle<const INPUT_LANES: usize, const OUTPUT_LANES: usize> {
    /// Map from the lanes of the input vector to the output vector.
    const INDEX: [usize; OUTPUT_LANES];

    /// Create a new vector from the lanes of `vector`.
    ///
    /// Lane `i`