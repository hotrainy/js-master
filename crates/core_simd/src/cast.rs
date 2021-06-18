use crate::simd::SimdElement;

/// Supporting trait for `Simd::cast`.  Typically doesn't need to be used directly.
///
/// # Safety
/// Implementing this trait asserts that the type is a valid vector element for the `simd_cast` or
/// `simd_as` intrinsics.
pub unsafe trait SimdCast: SimdElement {}

// Safety: primitive number typ