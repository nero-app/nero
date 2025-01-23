mod components;
mod macros;
mod pages;
mod types;
mod utils;

use pages::App;
use sycamore::render;

fn main() {
    console_error_panic_hook::set_once();

    render(|| App.into())
}
