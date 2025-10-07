mod processors;
pub mod server;
mod types;

use anyhow::{Result, anyhow};
use nero_wasm_host::{Metadata, semver::Version};
use nero_wit_process::WitProcessCtx;
use wasmtime::{Engine, component::Component};
use wasmtime_wasi::{ResourceTable, WasiCtx, WasiCtxView, WasiView};
use wasmtime_wasi_http::{WasiHttpCtx, WasiHttpView};

use crate::processors::{ProcessorPre, since_v0_1_0_draft};

struct WasmState {
    table: ResourceTable,
    ctx: WasiCtx,
    http_ctx: WasiHttpCtx,
    wit_process_ctx: WitProcessCtx,
}

impl Default for WasmState {
    fn default() -> Self {
        Self {
            table: ResourceTable::new(),
            ctx: WasiCtx::builder().build(),
            http_ctx: WasiHttpCtx::new(),
            wit_process_ctx: WitProcessCtx {},
        }
    }
}

impl WasiView for WasmState {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

impl WasiHttpView for WasmState {
    fn ctx(&mut self) -> &mut WasiHttpCtx {
        &mut self.http_ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub struct WasmProcessor {
    processor_pre: ProcessorPre,
    metadata: Metadata,
}

impl nero_wasm_host::WasmComponent for WasmProcessor {
    async fn instantiate_async(
        engine: &Engine,
        version: Version,
        component: &Component,
        metadata: Metadata,
    ) -> Result<Self> {
        let processor_pre = match version {
            v if v >= since_v0_1_0_draft::MIN_VER => {
                let linker = since_v0_1_0_draft::linker(engine)?;
                let pre = linker.instantiate_pre(component)?;
                Ok(ProcessorPre::V0_1_0_DRAFT(
                    since_v0_1_0_draft::ProcessorPre::new(pre)?,
                ))
            }
            _ => Err(anyhow!("unsupported processor version")),
        }?;

        Ok(Self {
            processor_pre,
            metadata,
        })
    }

    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
