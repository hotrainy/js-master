macro_rules! impl_to_bytes {
    { $ty:ty, $size:literal } => {
        impl<const LANES: usize> crate::simd::Simd<$ty, LANES>
        where
            crate::simd::LaneCount<LANES>: crate::simd::SupportedLaneCount,
            crate::simd::LaneCount<{{ $size * LANES }}>: crate::simd::SupportedLaneCount,
        {
            /// Return the memory representation of this integer as a byte array in native byte
            /// order.
            pub fn to_ne_bytes(self) -> crate::simd::Simd<u