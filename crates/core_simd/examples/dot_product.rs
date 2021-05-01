
// Code taken from the `packed_simd` crate
// Run this code with `cargo test --example dot_product`
//use std::iter::zip;

#![feature(array_chunks)]
#![feature(slice_as_chunks)]
// Add these imports to use the stdsimd library
#![feature(portable_simd)]
use core_simd::simd::*;

// This is your barebones dot product implementation:
// Take 2 vectors, multiply them element wise and *then*
// go along the resulting array and add up the result.
// In the next example we will see if there
//  is any difference to adding and multiplying in tandem.
pub fn dot_prod_scalar_0(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());

    a.iter().zip(b.iter()).map(|(a, b)| a * b).sum()
}

// When dealing with SIMD, it is very important to think about the amount
// of data movement and when it happens. We're going over simple computation examples here, and yet
// it is not trivial to understand what may or may not contribute to performance
// changes. Eventually, you will need tools to inspect the generated assembly and confirm your
// hypothesis and benchmarks - we will mention them later on.
// With the use of `fold`, we're doing a multiplication,
// and then adding it to the sum, one element from both vectors at a time.
pub fn dot_prod_scalar_1(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .fold(0.0, |a, zipped| a + zipped.0 * zipped.1)
}

// We now move on to the SIMD implementations: notice the following constructs:
// `array_chunks::<4>`: mapping this over the vector will let use construct SIMD vectors
// `f32x4::from_array`: construct the SIMD vector from a slice
// `(a * b).reduce_sum()`: Multiply both f32x4 vectors together, and then reduce them.
// This approach essentially uses SIMD to produce a vector of length N/4 of all the products,
// and then add those with `sum()`. This is suboptimal.
// TODO: ASCII diagrams