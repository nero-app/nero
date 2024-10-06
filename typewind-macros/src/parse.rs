use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, Data, DeriveInput, Fields};

pub fn impl_parse(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let variants = match &input.data {
        Data::Struct(data_struct) => {
            vec![generate_struct_variant(name, &data_struct.fields)]
        }
        Data::Enum(data_enum) => data_enum
            .variants
            .iter()
            .map(|v| generate_enum_variant(name, v))
            .collect(),
        _ => panic!("Parse can only be derived for structs and enums"),
    };

    quote! {
        impl syn::parse::Parse for #name {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let path: syn::Path = input.parse()?;
                let variant = path.segments.last().unwrap().ident.to_string();

                match variant.as_str() {
                    #(#variants)*
                    _ => Err(syn::Error::new(input.span(), "Unknown variant")),
                }
            }
        }
    }
    .into()
}

fn generate_struct_variant(name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Unnamed(fields_unnamed) if fields_unnamed.unnamed.len() == 1 => {
            let field_type = &fields_unnamed.unnamed.first().unwrap().ty;
            quote! {
                stringify!(#name) => {
                    let content;
                    syn::parenthesized!(content in input);

                    let value: #field_type = content.parse()?;
                    Ok(#name(value))
                },
            }
        }
        _ => panic!("Parse can only be derived for structs with a single unnamed field"),
    }
}

fn generate_enum_variant(name: &syn::Ident, variant: &syn::Variant) -> proc_macro2::TokenStream {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unnamed(fields_unnamed) => {
            let field_type = &fields_unnamed.unnamed.first().unwrap().ty;
            quote! {
                stringify!(#variant_ident) => {
                    let content;
                    syn::parenthesized!(content in input);

                    let value: #field_type = content.parse()?;
                    Ok(#name::#variant_ident(value))
                },
            }
        }
        Fields::Unit => quote! {
            stringify!(#variant_ident) => Ok(#name::#variant_ident),
        },
        Fields::Named(_) => panic!("Named fields are not supported in Parse derive"),
    }
}
