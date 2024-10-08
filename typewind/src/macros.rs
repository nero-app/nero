/// Registers modules and generates functions to collect all Typewind types
/// and convert instances, such as `FontFamily::Sans`, into Tailwind classes,
/// like `font-sans`.
macro_rules! register_mods {
    ($($mod_name:ident),*) => {
        $(pub mod $mod_name;)*

        fn get_all_types() -> Vec<String> {
            vec![
                $($mod_name::get_types_names(),)*
            ].concat()
        }

        fn convert_to_classes(instances: &[String]) -> Vec<String> {
            vec![
                $($mod_name::convert_to_classes(instances),)*
            ].concat()
        }
    };
}

/// Defines types to be converted into Tailwind classes.
///
/// This macro is used for final types that will be directly converted to Tailwind classes,
/// not for reusable intermediate types.
macro_rules! tailwind_types {
    ($($t:ident),+) => {
        pub(crate) fn get_types_names() -> Vec<String> {
            vec![$(stringify!($t).to_string(),)+]
        }

        pub(crate) fn convert_to_classes(instances: &[String]) -> Vec<String> {
            instances.iter()
                .filter_map(|expr_str| {
                    match expr_str.split(|c| c == ' ').next().unwrap_or("") {
                        $(
                            stringify!($t) => syn::parse_str::<$t>(expr_str).ok().map(|expr| expr.to_string()),
                        )+
                        _ => {
                            eprintln!("Failed to match type for expression: {}", expr_str);
                            None
                        }
                    }
                })
                .collect()
        }
    };
}
