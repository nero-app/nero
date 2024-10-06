use proc_macro::TokenStream;

mod classes;
mod display;
mod parse;

#[proc_macro_derive(ToClasses, attributes(tw))]
pub fn to_classes_derive(input: TokenStream) -> TokenStream {
    classes::impl_to_classes(input)
}

#[proc_macro_derive(Display, attributes(display))]
pub fn display_derive(input: TokenStream) -> TokenStream {
    display::impl_display(input)
}

#[proc_macro_derive(Parse)]
pub fn parse_derive(input: TokenStream) -> TokenStream {
    parse::impl_parse(input)
}
