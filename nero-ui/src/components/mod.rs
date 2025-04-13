mod button;
mod card;
mod icon;
mod list;
mod toolbar;
mod video;

pub use button::*;
pub use card::*;
pub use icon::*;
pub use list::*;
pub use toolbar::*;
pub use video::*;

use sycamore::web::{create_node_ref, events, GlobalProps};
use wasm_bindgen::JsCast;

pub trait OnReachBottom {
    /// Adds an event listener that triggers the provided function when the user scrolls to the bottom.
    ///
    /// # Arguments
    ///
    /// * `action`: A function that will be called when the bottom of the scrollable element is reached.
    fn on_reach_bottom<F>(self, action: F) -> Self
    where
        F: Fn() + 'static;
}

impl<T: GlobalProps> OnReachBottom for T {
    fn on_reach_bottom<F>(mut self, action: F) -> Self
    where
        F: Fn() + 'static,
    {
        let noderef = create_node_ref();

        let on_scroll = move |_| {
            let element = noderef.get().unchecked_into::<web_sys::HtmlElement>();
            let rest = element.scroll_height() as f64 - element.scroll_top() as f64;
            let is_at_bottom = (element.client_height() as f64 - rest).abs() < 1.0;

            if is_at_bottom {
                action();
            }
        };

        self = self.r#ref(noderef).on(events::scroll, on_scroll);
        self
    }
}
