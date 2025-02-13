use std::path::Path;

use anyhow::{Ok, Result};
use wasmtime::{
    Engine, Store,
    component::{Component, ResourceTable},
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::{extensions::WasmExtension, semver::SemanticVersion};

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
    pub async fn load_extension_async<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> wasmtime::Result<WasmExtension> {
        let path = path.as_ref();

        let wasm_bytes = std::fs::read(path)?;
        let version = Self::get_extension_version(&wasm_bytes)?;

        let component = Component::from_file(&self.engine, path)?;

        let store = Store::new(&self.engine, WasmState {
            table: ResourceTable::new(),
            ctx: WasiCtxBuilder::new().build(),
            http_ctx: WasiHttpCtx::new(),
        });

        let extension =
            WasmExtension::instantiate_async(&self.engine, store, version, &component).await?;

        Ok(extension)
    }

    // TODO
    #[allow(unused_variables)]
    fn get_extension_version(wasm_bytes: &[u8]) -> Result<SemanticVersion> {
        Ok(SemanticVersion::new(0, 0, 1))
    }
}

pub(crate) struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
}

impl WasiView for WasmState {
    fn table(&mut self) -> &mut wasmtime_wasi::ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.ctx
    }
}

impl WasiHttpView for WasmState {
    fn ctx(&mut self) -> &mut wasmtime_wasi_http::WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
