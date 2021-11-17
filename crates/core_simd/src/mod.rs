#[macro_use]
mod swizzle;

pub(crate) mod intrinsics;

#[cfg(feature = "generic_const_exprs")]
mod to_bytes;

mod alias;
mod cast;
mod elements;
mod eq;
