
pub mod array;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[macro_use]
pub mod biteq;

/// Specifies the default strategy for testing a type.
///
/// This strategy should be what "makes sense" to test.
pub trait DefaultStrategy {
    type Strategy: proptest::strategy::Strategy<Value = Self>;
    fn default_strategy() -> Self::Strategy;
}

macro_rules! impl_num {
    { $type:tt } => {
        impl DefaultStrategy for $type {
            type Strategy = proptest::num::$type::Any;
            fn default_strategy() -> Self::Strategy {
                proptest::num::$type::ANY
            }
        }
    }
}

impl_num! { i8 }
impl_num! { i16 }
impl_num! { i32 }
impl_num! { i64 }
impl_num! { isize }
impl_num! { u8 }
impl_num! { u16 }
impl_num! { u32 }
impl_num! { u64 }
impl_num! { usize }
impl_num! { f32 }
impl_num! { f64 }

impl<T> DefaultStrategy for *const T {
    type Strategy = proptest::strategy::Map<proptest::num::isize::Any, fn(isize) -> *const T>;
    fn default_strategy() -> Self::Strategy {
        fn map<T>(x: isize) -> *const T {
            x as _
        }
        use proptest::strategy::Strategy;
        proptest::num::isize::ANY.prop_map(map)
    }
}

impl<T> DefaultStrategy for *mut T {
    type Strategy = proptest::strategy::Map<proptest::num::isize::Any, fn(isize) -> *mut T>;
    fn default_strategy() -> Self::Strategy {
        fn map<T>(x: isize) -> *mut T {
            x as _
        }
        use proptest::strategy::Strategy;
        proptest::num::isize::ANY.prop_map(map)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl DefaultStrategy for u128 {
    type Strategy = proptest::num::u128::Any;
    fn default_strategy() -> Self::Strategy {
        proptest::num::u128::ANY
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl DefaultStrategy for i128 {
    type Strategy = proptest::num::i128::Any;
    fn default_strategy() -> Self::Strategy {
        proptest::num::i128::ANY
    }
}

#[cfg(target_arch = "wasm32")]
impl DefaultStrategy for u128 {
    type Strategy = crate::wasm::u128::Any;
    fn default_strategy() -> Self::Strategy {
        crate::wasm::u128::ANY
    }
}
