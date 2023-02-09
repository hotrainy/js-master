//! Generic-length array strategy.

// Adapted from proptest's array code
// Copyright 2017 Jason Lingle

use core::{marker::PhantomData, mem::MaybeUninit};
use proptest::{
    strategy::{NewTree, Strategy, ValueTree},
    test_runner::TestRunner,
};

#[must_use = "strategies do nothing unless used"]
#[derive(Clone, Copy, Debug)]
pub struct UniformArrayStrategy<S, T> {
    strategy: S,
    _marker: PhantomData<T>,
}

impl<S, T> UniformArrayStrategy<S, T> {
    pub const fn new(strategy: S) -> Self {
        Self {
            strategy,
            _marker: PhantomData,
        }
    }
}

pub struct ArrayValueTree<T> {
    tree: T,
    shrinker: usize,
    last_shrinker: Option<