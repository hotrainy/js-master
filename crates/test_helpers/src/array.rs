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
    last_shrinker: Option<usize>,
}

impl<T, S, const LANES: usize> Strategy for UniformArrayStrategy<S, [T; LANES]>
where
    T: core::fmt::Debug,
    S: Strategy<Value = T>,
{
    type Tree = ArrayValueTree<[S::Tree; LANES]>;
    type Value = [T; LANES];

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        let tree: [S::Tree; LANES] = unsafe {
            let mut tree: [MaybeUninit<S::Tree>; LANES] = MaybeUninit::uninit().assume_init();
            for t in tree.iter_mut() {
                *t = MaybeUninit::new(self.strategy.new_tree(runner)?)
            }
            core::mem::transmute_copy(&tree)
        };
        Ok(ArrayValueTree {
            tree,
            shrinker: 0,
            last_shrinker: None,
        })
    }
}

impl<T: ValueTree, const LANES: usize> ValueTree for ArrayValueTree<[T; LANES]> {
    type Value = [T::Value; LANES];

    fn current(&self) -> Self::Value {
        unsafe {
          