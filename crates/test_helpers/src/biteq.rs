//! Compare numeric types by exact bit value.

pub trait BitEq {
    fn biteq(&self, other