use leptos::{mount_to_body, view};
use typewind::{
    customization::{Color, ColorTone},
    typography::TextColor,
};

fn main() {
    console_error_panic_hook::set_once();

    let color = TextColor(Color::Red(ColorTone::_500)).to_string();
    mount_to_body(|| {
        view! {
            <h1 class={color}>"Hello World"</h1>
        }
    })
}
