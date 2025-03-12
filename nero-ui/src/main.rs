mod components;
mod extensions;
mod hooks;
mod macros;
mod pages;
mod utils;

use pages::App;
use sycamore::render;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn invoke_without_args(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

fn main() {
    console_error_panic_hook::set_once();

    render(|| App.into())
}
