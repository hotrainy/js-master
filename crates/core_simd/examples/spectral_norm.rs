#![feature(portable_simd)]

use core_simd::simd::*;

fn a(i: usize, j: usize) -> f64 {
    ((i + j) * (i + j + 1) / 2 + i + 1) as f64
}

fn mult_av(v: &[f64], out: &mut [f64]) {
    assert!(v.len() == out.len());
    assert!(v.len() % 2 == 0);

    for (i, out) i