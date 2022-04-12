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
    /// Lane `i` of the output is `vector[Self::INDEX[i]]`.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn swizzle<T>(vector: Simd<T, INPUT_LANES>) -> Simd<T, OUTPUT_LANES>
    where
        T: SimdElement,
        LaneCount<INPUT_LANES>: SupportedLaneCount,
        LaneCount<OUTPUT_LANES>: SupportedLaneCount,
    {
        // Safety: `vector` is a vector, and `INDEX_IMPL` is a const array of u32.
        unsafe { intrinsics::simd_shuffle(vector, vector, Self::INDEX_IMPL) }
    }
}

/// Create a vector from the elements of two other vectors.
pub trait Swizzle2<const INPUT_LANES: usize, const OUTPUT_LANES: usize> {
    /// Map from the lanes of the input vectors to the output vector
    const INDEX: [Which; OUTPUT_LANES];

    /// Create a new vector from the lanes of `first` and `second`.
    ///
    /// Lane `i` is `first[j]` when `Self::INDEX[i]` is `First(j)`, or `second[j]` when it is
    /// `Second(j)`.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    fn swizzle2<T>(
        first: Simd<T, INPUT_LANES>,
        second: Simd<T, INPUT_LANES>,
    ) -> Simd<T, OUTPUT_LANES>
    where
        T: SimdElement,
        LaneCount<INPUT_LANES>: SupportedLaneCount,
        LaneCount<OUTPUT_LANES>: SupportedLaneCount,
    {
        // Safety: `first` and `second` are vectors, and `INDEX_IMPL` is a const array of u32.
        unsafe { intrinsics::simd_shuffle(first, second, Self::INDEX_IMPL) }
    }
}

/// The `simd_shuffle` intrinsic expects `u32`, so do error checking and conversion here.
/// This trait hides `INDEX_IMPL` from the public API.
trait SwizzleImpl<const INPUT_LANES: usize, const OUTPUT_LANES: usize> {
    const INDEX_IMPL: [u32; OUTPUT_LANES];
}

impl<T, const INPUT_LANES: usize, const OUTPUT_LANES: usize> SwizzleImpl<INPUT_LANES, OUTPUT_LANES>
    for T
where
    T: Swizzle<INPUT_LANES, OUTPUT_LANES> + ?Sized,
{
    const INDEX_IMPL: [u32; OUTPUT_LANES] = {
        let mut output = [0; OUTPUT_LANES];
        let mut i = 0;
        while i < OUTPUT_LANES {
            let index = Self::INDEX[i];
            assert!(index as u32 as usize == index);
            assert!(index < INPUT_LANES, "source lane exceeds input lane count",);
            output[i] = index as u32;
            i += 1;
        }
        output
    };
}

/// The `simd_shuffle` intrinsic expects `u32`, so do error checking and conversion here.
/// This trait hides `INDEX_IMPL` from the public API.
trait Swizzle2Impl<const INPUT_LANES: usize, const OUTPUT_LANES: usize> {
    const INDEX_IMPL: [u32; OUTPUT_LANES];
}

impl<T, const INPUT_LANES: usize, const OUTPUT_LANES: usize> Swizzle2Impl<INPUT_LANES, OUTPUT_LANES>
    for T
where
    T: Swizzle2<INPUT_LANES, OUTPUT_LANES> + ?Sized,
{
    const INDEX_IMPL: [u32; OUTPUT_LANES] = {
        let mut output = [0; OUTPUT_LANES];
        let mut i = 0;
        while i < OUTPUT_LANES {
            let (offset, index) = match Self::INDEX[i] {
                Which::First(index) => (false, index),
                Which::Second(index) => (true, index),
            };
            assert!(index < INPUT_LANES, "source lane exceeds input lane count",);

            // lanes are indexed by the first vector, then second vector
            let index = if offset { index + INPUT_LANES } else { index };
            assert!(index as u32 as usize == index);
            output[i] = index as u32;
            i += 1;
        }
        output
    };
}

impl<T, const LANES: usize> Simd<T, LANES>
where
    T: SimdElement,
    LaneCount<LANES>: SupportedLaneCount,
{
    /// Reverse the order of the lanes in the vector.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    pub fn reverse(self) -> Self {
        const fn reverse_index<const LANES: usize>() -> [usize; LANES] {
            let mut index = [0; LANES];
            let mut i = 0;
            while i < LANES {
                index[i] = LANES - i - 1;
                i += 1;
            }
            index
        }

        struct Reverse;

        impl<const LANES: usize> Swizzle<LANES, LANES> for Reverse {
            const INDEX: [usize; LANES] = reverse_index::<LANES>();
        }

        Reverse::swizzle(self)
    }

    /// Rotates the vector such that the first `OFFSET` elements of the slice move to the end
    /// while the last `LANES - OFFSET` elements move to the front. After calling `rotate_lanes_left`,
    /// the element previously in lane `OFFSET` will become the first element in the slice.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    pub fn rotate_lanes_left<const OFFSET: usize>(self) -> Self {
        const fn rotate_index<const OFFSET: usize, const LANES: usize>() -> [usize; LANES] {
            let offset = OFFSET % LANES;
            let mut index = [0; LANES];
            let mut i = 0;
            while i < LANES {
                index[i] = (i + offset) % LANES;
                i += 1;
            }
            index
        }

        struct Rotate<const OFFSET: usize>;

        impl<const OFFSET: usize, const LANES: usize> Swizzle<LANES, LANES> for Rotate<OFFSET> {
            const INDEX: [usize; LANES] = rotate_index::<OFFSET, LANES>();
        }

        Rotate::<OFFSET>::swizzle(self)
    }

    /// Rotates the vector such that the first `LANES - OFFSET` elements of the vector move to
    /// the end while the last `OFFSET` elements move to the front. After calling `rotate_lanes_right`,
    /// the element previously at index `LANES - OFFSET` will become the first element in the slice.
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    pub fn rotate_lanes_right<const OFFSET: usize>(self) -> Self {
        const fn rotate_index<const OFFSET: usize, const LANES: usize>() -> [usize; LANES] {
            let offset = LANES - OFFSET % LANES;
            let mut index = [0; LANES];
            let mut i = 0;
            while i < LANES {
                index[i] = (i + offset) % LANES;
                i += 1;
            }
            index
        }

        struct Rotate<const OFFSET: usize>;

        impl<const OFFSET: usize, const LANES: usize> Swizzle<LANES, LANES> for Rotate<OFFSET> {
            const INDEX: [usize; LANES] = rotate_index::<OFFSET, LANES>();
        }

        Rotate::<OFFSET>::swizzle(self)
    }

    /// Interleave two vectors.
    ///
    /// The resulting vectors contain lanes taken alternatively from `self` and `other`, first
    /// filling the first result, and then the second.
    ///
    /// The reverse of this operation is [`Simd::deinterleave`].
    ///
    /// ```
    /// # #![feature(portable_simd)]
    /// # use core::simd::Simd;
    /// let a = Simd::from_array([0, 1, 2, 3]);
    /// let b = Simd::from_array([4, 5, 6, 7]);
    /// let (x, y) = a.interleave(b);
    /// assert_eq!(x.to_array(), [0, 4, 1, 5]);
    /// assert_eq!(y.to_array(), [2, 6, 3, 7]);
    /// ```
    #[inline]
    #[must_use = "method returns a new vector and does not mutate the original inputs"]
    pub fn interleave(self, other: Self) -> (Self, Self) {
        const fn interleave<const LANES: usize>(high: bool) -> [Which; LANES] {
            let mut idx = [Which::First(0); LANES];
            let mut i = 0;
            while i < LANES {
                // Treat the source as a concatenated vector
                let dst_index = if high { i + LANES } else { i };
                let src_index = dst_index / 2 + (dst_index % 2) * LANES;
                idx[i] = if src_index < LANES {
                    Which::First(src_index)
                } else {
                    Which::Second(src_index % LANES)
                };
                i += 1;
          