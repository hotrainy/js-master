use crate::simd::intrinsics;
use crate::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

/// Constructs a new SIMD vector by copying elements from selected lanes in other vectors.
///
/// When swizzling one vector, lanes are selected by a `const` array of `usize`,
/// like [`Swizzle`].
///
/// When swizzling two vectors, lanes are selected by 