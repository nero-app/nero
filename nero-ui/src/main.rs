use nero_components::{layout::HStack, Text};
use sycamore::render;
use typewind::{
    customization::{Color, ColorTone}, typography::{FontSize, TextColor}
};

fn main() {
    console_error_panic_hook::set_once();

    let color = TextColor(Color::Red(ColorTone::_500));
    let stack = HStack::new((
        Text::new("Hello,".to_owned())
            .color(color.clone())
            .font_size(FontSize::_3xl),
        Text::new("World!".to_owned()).color(color),
    ));

    render(|| stack.into())
}
