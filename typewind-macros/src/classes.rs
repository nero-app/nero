use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(tw))]
struct ToClassesOpts {
    behavior: Option<IncludeBehavior>,
}

#[derive(FromField, Default)]
#[darling(default, attributes(tw))]
struct FieldOpts {
    skip: bool,
    variant: bool,
}
enum IncludeBehavior {
    Variant,
    Skip,
}

impl FromMeta for IncludeBehavior {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "variant" => Ok(IncludeBehavior::Variant),
            "skip" => Ok(IncludeBehavior::Skip),
            _ => Err(darling::Error::unknown_value(value)),
        }
    }
}

pub fn impl_to_classes(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let opts = ToClassesOpts::from_derive_input(&input).unwrap();
    let include_behavior = opts.behavior.unwrap_or(IncludeBehavior::Variant);

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("ToClasses derive macro only supports structs with named fields"),
        },
        _ => panic!("ToClasses derive macro only supports structs"),
    };

    let class_fields = fields.iter().filter_map(|f| {
        let field_name = &f.ident;
        let opts = FieldOpts::from_field(f).expect("Invalid field options");
        match !opts.skip && (opts.variant || !matches!(include_behavior, IncludeBehavior::Skip)) {
            true => Some(quote! { self.#field_name.to_string(), }),
            false => None,
        }
    });

    quote! {
        impl #impl_generics ToClasses for #name #ty_generics #where_clause {
            fn classes(&self) -> String {
                let classes: Vec<String> = vec![
                    #( #class_fields )*
                ];

                classes.join(" ")
            }
        }
    }
    .into()
}
