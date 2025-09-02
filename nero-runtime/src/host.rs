use std::path::Path;

use anyhow::Result;
use wasm_metadata::Payload;
use wasmtime::{Engine, component::Component};

use crate::{WasmExtension, semver::SemanticVersion};

pub struct WasmHost {
    engine: Engine,
}

impl Default for WasmHost {
    fn default() -> Self {
        Self {
            engine: {
                let mut config = wasmtime::Config::new();
                config.async_support(true);
                config.wasm_component_model(true);
                wasmtime::Engine::new(&config).unwrap()
            },
        }
    }
}

impl WasmHost {
    pub async fn load_extension_async<T: WasmExtension, P: AsRef<Path>>(
        &self,
        path: P,
    ) -> wasmtime::Result<T> {
        let path = path.as_ref();

        let wasm_bytes = std::fs::read(path)?;
        let version = Self::get_extension_version(&wasm_bytes)?;
        let component = Component::from_file(&self.engine, path)?;
        let metadata = match Payload::from_binary(&wasm_bytes)? {
            Payload::Component { metadata, .. } => metadata,
            Payload::Module(..) => unreachable!(),
        };

        let extension = T::instantiate_async(&self.engine, version, &component, metadata).await?;

        Ok(extension)
    }

    // TODO:
    #[allow(unused_variables)]
    fn get_extension_version(wasm_bytes: &[u8]) -> Result<SemanticVersion> {
        Ok(SemanticVersion::new(0, 1, 0))
    }
}
