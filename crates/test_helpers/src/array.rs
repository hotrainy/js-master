//! Generic-length array strategy.

// Adapted from proptest's array code
// Copyright 2017 Jason Lingle

use core::{marker::PhantomData, mem::MaybeUninit};
use proptest::{
    strategy::{NewTree, Strategy, ValueTree},
    test_runner::TestRunner,
};

#[must_use = "strategies do nothing unless u