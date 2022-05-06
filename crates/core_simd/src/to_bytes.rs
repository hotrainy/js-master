macro_rules! impl_to_bytes {
    { $ty:ty, $size:literal } => {
        impl<const LANES: usize> crate::simd::Simd<$ty