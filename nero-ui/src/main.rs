mod components;
mod macros;
mod pages;
mod utils;

use pages::{BaseLayout, SeriesPage};
use sycamore::render;

fn main() {
    console_error_panic_hook::set_once();

    render(|| BaseLayout::new(SeriesPage).into())
}
