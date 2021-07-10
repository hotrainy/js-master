use crate::simd::SimdElement;

/// Supporting trait for `Simd::cast`.  Typically doesn't need to be used directly.
///
/// # Safety
/// Implementing this trait asserts that the type is a valid vector element for the `simd_cast` or
/// `simd_as` intrinsics.
pub unsafe trait SimdCast: SimdElement {}

// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for i8 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for i16 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for i32 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for i64 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for isize {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for u8 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for u16 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for u32 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for u64 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for usize {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for f32 {}
// Safety: primitive number types can be cast to other primitive number types
unsafe impl SimdCast for f64 {}

/// Supporting trait for `Simd::cast_ptr`.  Typically doesn't need to be used directly.
///
/// # Safety
/// Implementing this trait asserts that the type is a valid vector element for the `simd_c