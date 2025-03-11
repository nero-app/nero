mod components;
mod extensions;
mod hooks;
mod macros;
mod pages;
mod utils;

use pages::App;
use sycamore::render;

fn main() {
    console_error_panic_hook::set_once();

    render(|| App.into())
}
