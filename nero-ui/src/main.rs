mod macros;

use sycamore::{
    render,
    web::{tags::h1, GlobalProps, HtmlGlobalAttributes},
};
use typewind::typography::{FontFamily, FontSize, TextColor};

fn main() {
    console_error_panic_hook::set_once();

    let title = h1().children("Hello, World!").class(format2!(
        TextColor::Red500,
        FontFamily::Mono,
        FontSize::_3xl,
    ));

    render(|| title.into())
}
