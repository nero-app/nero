/// Creates a string from the given arguments that implements [`std::fmt::Display`].
///
/// Unlike `format!`, it automatically concatenates the elements with a space
/// separator and does not require manually specifying `{}` for each argument.
#[macro_export]
macro_rules! format2 {
    ($($arg:expr),+ $(,)?) => {{
        [$(format!("{}", $arg)),+].join(" ")
    }};
}
