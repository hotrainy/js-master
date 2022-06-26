macro_rules! mask_tests {
    { $vector:ident, $lanes:literal } => {
        #[cfg(test)]
        mod $vector {
            use core_simd::simd::$vector as Vector;
            const LANES: usize = $lanes;

            #[cfg(target_arch = "wasm32")]
            use wasm_bindgen_test::*;

            #[cfg(target_arch = "wasm32")]
            wasm_bindgen_test_configure!(run_in_browser);

            fn from_slice(slice: &[bool]) -> Vector {
                let mut value = Vector::default();
                for (i, b) in slice.iter().take(LANES).enumerate() {
                    value.set(i, *b);
        