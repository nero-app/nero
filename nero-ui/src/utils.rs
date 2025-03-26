use nero_extensions::anyhow::{anyhow, Result};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::web::{window, View};
use sycamore_router::navigate_no_history;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::{invoke, invoke_without_args};

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

pub async fn invoke_and_parse<T>(cmd: &str, args: Option<serde_json::Value>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let res = match args {
        Some(args_value) => {
            let js_value = to_value(&args_value).unwrap_throw();
            invoke(cmd, js_value)
                .await
                .map_err(|e| anyhow!("Error invoking {}: {:?}", cmd, e))?
        }
        None => invoke_without_args(cmd)
            .await
            .map_err(|e| anyhow!("Error invoking {}: {:?}", cmd, e))?,
    };

    from_value::<T>(res).map_err(|e| anyhow!("Error parsing result from {}: {:?}", cmd, e))
}
