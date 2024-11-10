use glob::{glob_with, MatchOptions};
use quote::ToTokens;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use syn::{parse::Parser, punctuated::Punctuated, Expr, ExprCall, ExprMacro, Token};

pub mod customization;
#[macro_use]
mod macros;

pub use typewind_macros::ToClasses;

register_mods!(
    layout,
    flexbox_grid,
    spacing,
    sizing,
    typography,
    backgrounds,
    borders,
    effects,
    filters,
    tables,
    transitions_animation
);

/// Trait for converting types into a string of tailwind classes.
pub trait ToClasses {
    fn classes(&self) -> String;
}

struct Visitor {
    instances: HashSet<String>,
    target_types: Vec<String>,
}

impl Visitor {
    fn new(target_types: Vec<String>) -> Self {
        Self {
            instances: HashSet::new(),
            target_types,
        }
    }

    fn is_target_type(&self, type_str: &str) -> bool {
        self.target_types.iter().any(|t| type_str.contains(t))
    }

    fn visit_file(&mut self, file_path: &Path) -> syn::Result<()> {
        let content = fs::read_to_string(file_path).unwrap();

        let ast = syn::parse_file(&content)?;
        syn::visit::visit_file(self, &ast);

        Ok(())
    }
}

impl<'ast> syn::visit::Visit<'ast> for Visitor {
    fn visit_expr(&mut self, i: &'ast syn::Expr) {
        match i {
            Expr::Macro(ExprMacro { mac, .. }) => {
                let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
                let args = parser.parse2(mac.tokens.clone()).unwrap_or_default();

                args.iter().for_each(|arg| {
                    let arg_str = arg.to_token_stream().to_string();
                    if self.is_target_type(&arg_str) {
                        self.instances.insert(arg_str);
                    }
                    self.visit_expr(arg);
                });
            }
            Expr::Call(ExprCall { func, args, .. }) => {
                let func_str = func.to_token_stream().to_string();
                if self.is_target_type(&func_str) {
                    self.instances.insert(i.to_token_stream().to_string());
                }
                args.iter().for_each(|a| {
                    self.visit_expr(a);
                });
            }
            Expr::Path(expr_path) => {
                let path_str = expr_path.to_token_stream().to_string();
                if self.is_target_type(&path_str) && path_str.contains("::") {
                    self.instances.insert(i.to_token_stream().to_string());
                }
            }
            _ => {}
        }
        syn::visit::visit_expr(self, i);
    }
}

fn iter_files(patterns: &[&str]) -> Vec<PathBuf> {
    let match_options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let mut all_files: HashSet<PathBuf> = HashSet::new();
    let mut excluded_files: HashSet<PathBuf> = HashSet::new();

    patterns.iter().for_each(|pattern| {
        let is_negative = pattern.starts_with('!');
        let actual_pattern = pattern.strip_prefix('!').unwrap_or(pattern);

        if let Ok(entries) = glob_with(actual_pattern, match_options) {
            entries.filter_map(Result::ok).for_each(|entry| {
                match is_negative {
                    true => excluded_files.insert(entry),
                    false => all_files.insert(entry),
                };
            });
        }
    });

    all_files
        .difference(&excluded_files)
        .filter(|&path| path.is_file())
        .cloned()
        .collect()
}

/// Generates the final Tailwind classes from the Typewind types.
///
/// # Arguments
/// - `output_file` - A string slice that holds the path where the converted Tailwind classes will be saved.
///     This file must be linked to `tailwind.config.js` in order for Tailwind to generate the
///     necessary styles.
/// - `content` - A list of string slices containing paths to files that contain typewind types.
///     This is similar to the `content` field in Tailwind's configuration, supporting patterns like `./src/**/*.rs`,
///     and negated patterns like `!./src/lib.rs` to exclude files from the build.
pub fn build(output_file: &str, content: &[&str]) -> std::io::Result<()> {
    let target_types = get_all_types();
    let mut visitor = Visitor::new(target_types);

    iter_files(content).iter().for_each(|file_path| {
        if let Err(e) = visitor.visit_file(file_path) {
            eprintln!("Error processing file {:?}: {}", file_path, e);
        }
    });

    let instances = visitor.instances.into_iter().collect::<Vec<_>>();
    let classes = convert_to_classes(&instances);

    let mut file = File::create(output_file)?;
    file.write_all(classes.join("\n").as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        customization::{Color, ColorTone},
        typography::{FontFamily, TextColor},
        ToClasses,
    };
    use syn::parse_str;

    #[test]
    fn test_text_color_parse_full_path() {
        let input = "typewind::typography::TextColor(typewind::customization::Color::Red(typewind::customization::ColorTone::_500))";
        let result = parse_str::<TextColor>(input);

        assert!(
            result.is_ok(),
            "Failed to parse TextColor: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), TextColor(Color::Red(ColorTone::_500)));
    }

    #[test]
    fn test_font_family_parse_full_path() {
        let input = "typewind::typography::FontFamily::Mono";
        let result = parse_str::<FontFamily>(input);

        assert!(
            result.is_ok(),
            "Failed to parse FontFamily with full path: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), FontFamily::Mono);
    }

    #[test]
    fn test_font_family_parse_typography_path() {
        let input = "typography::FontFamily::Sans";
        let result = parse_str::<FontFamily>(input);

        assert!(
            result.is_ok(),
            "Failed to parse FontFamily with typography path: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), FontFamily::Sans);
    }

    #[test]
    fn test_font_family_parse_short_path() {
        let input = "FontFamily::Sans";
        let result = parse_str::<FontFamily>(input);

        assert!(
            result.is_ok(),
            "Failed to parse FontFamily with short path: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), FontFamily::Sans);
    }

    #[test]
    fn test_font_family_variant_parse() {
        let input = "Sans";
        let result = parse_str::<FontFamily>(input);

        assert!(
            result.is_ok(),
            "Failed to parse FontFamily Sans type: {:?}",
            result.err()
        );
        assert_eq!(result.unwrap(), FontFamily::Sans);
    }

    #[test]
    fn test_font_family_to_class() {
        let input = ["FontFamily::Sans", "FontFamily::Serif", "FontFamily::Mono"];
        let expected = ["font-sans", "font-serif", "font-mono"];

        let result = input
            .into_iter()
            .filter_map(|s| parse_str::<FontFamily>(s).ok())
            .map(|f| f.to_string())
            .collect::<Vec<String>>();

        assert_eq!(result, expected, "FontFamily to class conversion failed");
    }

    #[test]
    fn test_to_classes_derive_with_variant_behavior() {
        #[allow(dead_code)]
        #[derive(ToClasses)]
        #[tw(behavior = "variant")]
        struct Text {
            #[tw(skip)]
            text: String,
            font_family: FontFamily,
            color: TextColor,
        }

        let text = Text {
            text: "Hello World".to_owned(),
            font_family: FontFamily::Sans,
            color: TextColor(Color::Green(ColorTone::_300)),
        };

        assert_eq!(text.classes(), "font-sans text-green-300");
    }

    #[test]
    fn test_to_classes_derive_with_skip_behavior() {
        #[allow(dead_code)]
        #[derive(ToClasses)]
        #[tw(behavior = "skip")]
        struct Text {
            text: String,
            #[tw(variant)]
            font_family: FontFamily,
            #[tw(variant)]
            color: TextColor,
        }

        let text = Text {
            text: "Hello World".to_owned(),
            font_family: FontFamily::Sans,
            color: TextColor(Color::Green(ColorTone::_300)),
        };

        assert_eq!(text.classes(), "font-sans text-green-300");
    }
}
