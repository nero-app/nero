use app::App;
use leptos::{mount_to_body, view};

mod app;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
