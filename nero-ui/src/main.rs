mod components;
mod macros;
mod pages;
mod utils;

use sycamore::{
    render,
    web::{tags::h1, GlobalProps},
};

fn main() {
    console_error_panic_hook::set_once();

    let title = h1().children("Hello, World!");

    render(|| title.into())
}
