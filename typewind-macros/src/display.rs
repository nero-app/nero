use darling::{FromDeriveInput, FromMeta, FromVariant};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(display))]
struct DisplayOpts {
    prefix: Option<String>,
    #[darling(multiple)]
    replace: Vec<Replace>,
}

#[derive(FromVariant, Default)]
#[darling(default, attributes(display))]
struct VariantOpts {
    no_prefix: Option<bool>,
}

#[derive(FromMeta, Default)]
struct Replace {
    from: String,
    to: String,
}

pub fn impl_display(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = DisplayOpts::from_derive_input(&input).expect("Invalid display options");

    let name = &input.ident;
    let implementation = match &input.data {
        Data::Struct(data) => generate_struct_impl(&data.fields, &opts),
        Data::Enum(data) => generate_enum_impl(data, &opts),
        _ => panic!("Display can only be derived for structs and enums"),
    };

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #implementation)
            }
        }
    }
    .into()
}

fn generate_struct_impl(fields: &Fields, opts: &DisplayOpts) -> proc_macro2::TokenStream {
    let body = match fields {
        Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
            quote! { self.0.to_string() }
        }
        _ => panic!("Display can only be derived for structs with a single unnamed field"),
    };

    match opts.prefix {
        Some(ref prefix) => quote! { format!("{}-{}", #prefix, #body) },
        None => body,
    }
}

fn generate_enum_impl(data: &syn::DataEnum, opts: &DisplayOpts) -> proc_macro2::TokenStream {
    let match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_opts = VariantOpts::from_variant(variant).expect("Invalid variant options");

        let variant_str = opts.replace.iter().fold(
            variant_name.to_string().trim_start_matches('_').to_string(),
            |acc, replace| acc.replace(&replace.from, &replace.to),
        );
        let kebab_variant = to_kebab_case(&variant_str);

        let formatted_variant = if variant_opts.no_prefix.unwrap_or_default() {
            kebab_variant.to_string()
        } else {
            match opts.prefix {
                Some(ref prefix) => format!("{}-{}", prefix, kebab_variant),
                None => kebab_variant.to_string(),
            }
        };

        match &variant.fields {
            Fields::Unnamed(_) => quote! {
                Self::#variant_name(inner) => format!("{}-{}", #formatted_variant, inner)
            },
            Fields::Unit => quote! { Self::#variant_name => #formatted_variant.to_string() },
            Fields::Named(_) => panic!("Named fields are not supported in Display derive"),
        }
    });

    quote! { match self { #(#match_arms,)* } }
}

fn to_kebab_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .fold(String::new(), |mut kebab, (i, ch)| {
            if i > 0 {
                let prev_ch = s.chars().nth(i - 1).unwrap();
                if ch.is_uppercase() || ch.is_numeric() && prev_ch.is_alphabetic() {
                    kebab.push('-');
                }
            }
            kebab.push(ch.to_ascii_lowercase());
            kebab
        })
}
