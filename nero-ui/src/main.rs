use leptos::{
    html::{h1, ElementChild},
    mount::mount_to_body,
    prelude::ClassAttribute,
};
use nero_components::{layout::Layout, IntoComponent};
use typewind::{
    customization::{Color, ColorTone},
    typography::TextColor,
};

fn main() {
    console_error_panic_hook::set_once();

    let color = TextColor(Color::Red(ColorTone::_500)).to_string();
    let stack = Layout::h_stack((
        h1().class(color.clone()).child("Hello,"),
        h1().class(color).child("World!"),
    ));

    mount_to_body(|| stack.into_component())
}
