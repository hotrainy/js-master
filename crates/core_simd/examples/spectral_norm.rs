#![feature(portable_simd)]

use core_simd::simd::*;

fn a(i: usize, j: usize) -> f64 {
    ((i + j) * (i + j + 1) / 2 + i + 1) as f64
}

fn mult_av(v: &[f64], out: &mut [f64]) {
    assert!(v.len() == out.len());
    assert!(v.len() % 2 == 0);

    for (i, out) in out.iter_mut().enumerate() {
        let mut sum = f64x2::splat(0.0);

        let mut j = 0;
        while j < v.len() {
            let b = f64x2::from_slice(&v[j..]);
            let a = f64x2::from_array([a(i, j), a(i, j + 1)]);
            sum += b / a;
            j += 2
        }
        *out = sum.reduce_sum();
    }
}

fn mult_atv(v: &[f64], out: &mut [f64]) {
    assert!(v.len() == out.len());
    assert!(v.len() % 2 == 0);

    for (i, out) in out.iter_mut().enumerate() {
        let mut sum = f64x2::splat(0.0);

        let mut j = 0;
        while j < v.len() {
            let b = f64x2::from_slice(&v[j..]);
            let a = f64x2::from_array([a(j, i), a(j + 1, i)]);
            sum += b / a;
            j += 2
        }
        *out = sum.reduce_sum();
    }