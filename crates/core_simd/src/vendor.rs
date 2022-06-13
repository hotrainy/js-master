/// Provides implementations of `From<$a> for $b` and `From<$b> for $a` that transmutes the value.
#[allow(unused)]
macro_rules! from_transmute {
    { unsafe $a:ty => $b:ty } => {
        from_transmute!{ @impl $a => $b }
        from_transmute!{ @impl $b => $a }
    };
    { @impl $from:ty => $to:ty } => {
        impl core::convert::From<$from> for $to {
            #[inline]
            fn from(value: $from) -> $to {
                // Safety: transmuting between vectors is safe, but the caller of this macro
                // checks the invariants
                unsafe { core::mem::transmute(value) }
            }
        }
    };
}
