use sycamore::web::{window, View};
use sycamore_router::navigate_no_history;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

pub trait ViewBuilder: Sized {
    fn map<U>(self, f: impl FnOnce(Self) -> U) -> U {
        f(self)
    }

    fn when(self, condition: bool, then: impl FnOnce(Self) -> Self) -> Self {
        self.map(|this| if condition { then(this) } else { this })
    }

    fn when_some<T>(self, option: Option<T>, then: impl FnOnce(Self, T) -> Self) -> Self {
        self.map(|this| {
            if let Some(value) = option {
                then(this, value)
            } else {
                this
            }
        })
    }
}

impl<T: Into<View>> ViewBuilder for T {}

pub fn navigate_with_state(url: &str, data: &JsValue) {
    let history = window().history().unwrap_throw();
    history
        .push_state_with_url(data, "", Some(url))
        .unwrap_throw();
    navigate_no_history(url);
}
