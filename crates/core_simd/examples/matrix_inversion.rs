
//! 4x4 matrix inverse
// Code ported from the `packed_simd` crate
// Run this code with `cargo test --example matrix_inversion`
#![feature(array_chunks, portable_simd)]
use core_simd::simd::*;
use Which::*;

// Gotta define our own 4x4 matrix since Rust doesn't ship multidim arrays yet :^)
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix4x4([[f32; 4]; 4]);

#[allow(clippy::too_many_lines)]
pub fn scalar_inv4x4(m: Matrix4x4) -> Option<Matrix4x4> {
    let m = m.0;

    #[rustfmt::skip]
    let mut inv = [
        // row 0:
        [
            // 0,0:
            m[1][1] * m[2][2] * m[3][3] -
            m[1][1] * m[2][3] * m[3][2] -
            m[2][1] * m[1][2] * m[3][3] +
            m[2][1] * m[1][3] * m[3][2] +
            m[3][1] * m[1][2] * m[2][3] -
            m[3][1] * m[1][3] * m[2][2],
            // 0,1:
           -m[0][1] * m[2][2] * m[3][3] +
            m[0][1] * m[2][3] * m[3][2] +
            m[2][1] * m[0][2] * m[3][3] -
            m[2][1] * m[0][3] * m[3][2] -
            m[3][1] * m[0][2] * m[2][3] +
            m[3][1] * m[0][3] * m[2][2],
            // 0,2:
            m[0][1] * m[1][2] * m[3][3] -
            m[0][1] * m[1][3] * m[3][2] -
            m[1][1] * m[0][2] * m[3][3] +
            m[1][1] * m[0][3] * m[3][2] +
            m[3][1] * m[0][2] * m[1][3] -
            m[3][1] * m[0][3] * m[1][2],
            // 0,3:
           -m[0][1] * m[1][2] * m[2][3] +
            m[0][1] * m[1][3] * m[2][2] +
            m[1][1] * m[0][2] * m[2][3] -
            m[1][1] * m[0][3] * m[2][2] -
            m[2][1] * m[0][2] * m[1][3] +
            m[2][1] * m[0][3] * m[1][2],
        ],
        // row 1
        [
            // 1,0:
           -m[1][0] * m[2][2] * m[3][3] +
            m[1][0] * m[2][3] * m[3][2] +
            m[2][0] * m[1][2] * m[3][3] -