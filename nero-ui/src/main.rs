use leptos::{
    html::{h1, ElementChild},
    mount::mount_to_body,
    prelude::ClassAttribute,
};
use typewind::{
    customization::{Color, ColorTone},
    typography::TextColor,
};

fn main() {
    console_error_panic_hook::set_once();

    let color = TextColor(Color::Red(ColorTone::_500)).to_string();
    mount_to_body(|| h1().class(color).child("Hello, world!"))
}
