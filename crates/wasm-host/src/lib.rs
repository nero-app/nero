mod host;
pub mod manager;

pub use host::WasmHost;
pub use semver;
pub use wasm_metadata::Metadata;

use anyhow::Result;
use semver::Version;
use wasmtime::{Engine, component::Component};

pub trait WasmComponent: Sized {
    fn instantiate_async(
        engine: &Engine,
        version: Version,
        component: &Component,
        metadata: Metadata,
    ) -> impl Future<Output = Result<Self>>;

    fn metadata(&self) -> &Metadata;
}
