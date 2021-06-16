
macro_rules! number {
    { 1 } => { "one" };
    { 2 } => { "two" };
    { 4 } => { "four" };
    { 8 } => { "eight" };
    { $x:literal } => { stringify!($x) };
}

macro_rules! plural {
    { 1 } => { "" };
    { $x:literal } => { "s" };
}

macro_rules! alias {
    {
        $(
            $element_ty:ty = {
                $($alias:ident $num_elements:tt)*
            }
        )*
    } => {
        $(