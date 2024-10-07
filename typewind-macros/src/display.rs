use darling::{FromDeriveInput, FromMeta};
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
        Data::Struct(data) => generate_struct_impl(&data.fields),
        Data::Enum(data) => generate_enum_impl(data, &opts),
        _ => panic!("Display can only be derived for structs and enums"),
    };

    let body = match opts.prefix {
        Some(prefix) => quote! { format!("{}-{}", #prefix, #implementation) },
        None => implementation,
    };

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #body)
            }
        }
    }
    .into()
}

fn generate_struct_impl(fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
            quote! { self.0.to_string() }
        }
        _ => panic!("Display can only be derived for structs with a single unnamed field"),
    }
}

fn generate_enum_impl(data: &syn::DataEnum, opts: &DisplayOpts) -> proc_macro2::TokenStream {
    let match_arms = data.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_str = opts.replace.iter().fold(
            variant_name.to_string().trim_start_matches('_').to_string(),
            |acc, replace| acc.replace(&replace.from, &replace.to)
        );
        let kebab_variant = to_kebab_case(&variant_str);

        match &variant.fields {
            Fields::Unnamed(_) => quote! {
                Self::#variant_name(inner) => format!("{}-{}", #kebab_variant, inner)
            },
            Fields::Unit => quote! { Self::#variant_name => #kebab_variant.to_string() },
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
