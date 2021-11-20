#[macro_use]
mod swizzle;

pub(crate) mod intrinsics;

#[cfg(feature = "generic_const_exprs")]
mod to_bytes;

mod alias;
mod cast;
mod elements;
mod eq;
mod fmt;
mod iter;
mod lane_count;
mod masks;
mod ops;
mod ord;
mod select;
mod vector;
mod vendor;

#[doc = include_str!("core_simd_docs.md")]
pub mod simd {
    pub(crate) use crate::core_simd: