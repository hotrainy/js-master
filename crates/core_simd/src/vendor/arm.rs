#![allow(unused)]
use crate::simd::*;

#[cfg(target_arch = "arm")]
use core::arch::arm::*;

#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "arm", target_feature = "v7"