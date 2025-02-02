#[macro_export]
macro_rules! tw {
    ($first:literal) => {
        $first
    };
    ($first:expr) => {
        rustwind::const_format::formatc!("{}", $first)
    };
    ($first:literal, $($rest:tt)*) => {
        rustwind::const_format::formatc!("{} {}", $first, $crate::tw!($($rest)*))
    };
    ($first:expr, $($rest:tt)*) => {
        rustwind::const_format::formatc!("{} {}", $first, $crate::tw!($($rest)*))
    };
}
