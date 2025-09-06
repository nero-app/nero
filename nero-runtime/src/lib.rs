mod host;
pub mod manager;
pub mod semver;

pub use host::WasmHost;
pub use wasm_metadata::Metadata;

use anyhow::Result;
use wasmtime::{Engine, component::Component};

use crate::semver::SemanticVersion;

pub trait WasmComponent: Sized {
    fn instantiate_async(
        engine: &Engine,
        version: SemanticVersion,
        component: &Component,
        metadata: Metadata,
    ) -> impl Future<Output = Result<Self>>;

    fn metadata(&self) -> &Metadata;
}
