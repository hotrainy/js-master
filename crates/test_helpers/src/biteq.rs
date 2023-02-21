//! Compare numeric types by exact bit value.

pub trait BitEq {
    fn biteq(&self, other: &Self) -> bool;
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result;
}

impl BitEq for bool {
    fn biteq(&self, other: &Self) -> bool {
        self == other
    }

    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

macro_rules! impl_integer_biteq {
    { $($type:ty),* } => {
        $(
        impl BitEq for $type {
            fn biteq(&self, other: &Self) -> bool {
                self == other
            }

            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{:?} ({:x})", self, self)
            }
        }
        )*
    };
}

impl_integer_biteq! { u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize }

macro_rules! impl_float_biteq {
    { $($type:ty),* } => {
        $(
        impl BitEq for $type {
            fn biteq(&self, other: &Self) -> bool {
                if self.is_nan() && other.is_nan() {
              