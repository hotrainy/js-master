//! Compare numeric types by exact bit value.

pub trait BitEq {
    fn biteq(&self, other: &Self) -> bool;
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result;
}

impl BitEq for bool {
    fn biteq(&self, other: &Self) -> bool {
        self == other
    }

    fn fmt(&self,