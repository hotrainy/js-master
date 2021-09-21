//! This module contains the LLVM intrinsics bindings that provide the functionality for this
//! crate.
//!
//! The LLVM assembly language is documented here: <https://llvm.org/docs/LangRef.html>
//!
//! A quick glossary of jargon that may appear in this module, mostly paraphrasing LLVM's LangRef:
//! - poison: "undefined behavior as a value". specifically, it is like uninit memory (such as padding bytes). it is "safe" to create poison, BUT
//!   poison MUST NOT be observed from safe code, as operations on poison return poison, like NaN. unlike NaN, which has defined comparisons,
//!   poison is neither true nor false, and LLVM may also convert it to undef (at which point it is both). so, it can't be conditioned on, either.
//! - undef: "a value that is every value". functionally like poison, insofar as Rust is 