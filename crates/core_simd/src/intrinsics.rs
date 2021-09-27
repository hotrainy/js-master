//! This module contains the LLVM intrinsics bindings that provide the functionality for this
//! crate.
//!
//! The LLVM assembly language is documented here: <https://llvm.org/docs/LangRef.html>
//!
//! A quick glossary of jargon that may appear in this module, mostly paraphrasing LLVM's LangRef:
//! - poison: "undefined behavior as a value". specifically, it is like uninit memory (such as padding bytes). it is "safe" to create poison, BUT
//!   poison MUST NOT be observed from safe code, as operations on poison return poison, like NaN. unlike NaN, which has defined comparisons,
//!   poison is neither true nor false, and LLVM may also convert it to undef (at which point it is both). so, it can't be conditioned on, either.
//! - undef: "a value that is every value". functionally like poison, insofar as Rust is concerned. poison may become this. note:
//!   this means that division by poison or undef is like division by zero, which means it inflicts...
//! - "UB": poison and undef cover most of what people call "UB". "UB" means this operation immediately invalidates the program:
//!   LLVM is allowed to lower it to `ud2` or other opcodes that may cause an illegal instruction exception, and this is the "good end".
//!   The "bad end" is that LLVM may reverse time to the moment control flow diverged on a path towards undefined behavior,
//!   and destroy the other branch, potentially deleting safe code and violating Rust's `unsafe` contract.
//!
//! Note that according to LLVM, vectors are not arrays, but they are equivalent when stored to and loaded from memory.
//!
//! Unless stated otherwise, all intrinsics for binary operations require SIMD vectors of equal types and lengths.

// These intrinsics aren't linked directly from LLVM and are mostly undocumented, however they are
// mostly lowered to the matching LLVM instructions by the compiler in a fairly straightforward manner.
// The associated LLVM instruction or intrinsic is documented alongside each Rust intrinsic function.
extern "platform-intrinsic" {
    /// add/fadd
    pub(crate) fn simd_add<T>(x: T, y: T) -> T;

    /// sub/