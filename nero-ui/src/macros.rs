#[macro_export]
macro_rules! tw {
    ($first:literal) => {
        $first
    };
    ($first:path) => {
        ($first).as_class()
    };
    ($first:expr) => {
        $first
    };
    ($first:literal, $($rest:tt)*) => {
        rustwind::const_format::concatcp!($first, " ", $crate::tw!($($rest)*))
    };
    ($first:path, $($rest:tt)*) => {
        rustwind::const_format::concatcp!(($first).as_class(), " ", $crate::tw!($($rest)*))
    };
    ($first:expr, $($rest:tt)*) => {
        rustwind::const_format::concatcp!($first, " ", $crate::tw!($($rest)*))
    };
}
