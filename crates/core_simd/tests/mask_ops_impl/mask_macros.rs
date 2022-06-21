macro_rules! mask_tests {
    { $vector:ident, $lanes:literal } => {
        #[cfg(test)]
        mod $vector {
            use core_simd::simd::$vector as Vector;
            const LANES: usize = $lanes;

            #[cfg(target_ar