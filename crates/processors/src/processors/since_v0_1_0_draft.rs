use anyhow::Result;
use nero_wasm_host::semver::SemanticVersion;
use nero_wit_process::WitProcess;
use wasmtime::{
    Engine,
    component::{Linker, bindgen},
};

use self::nero::processor::ffmpeg_sidecar;
use crate::WasmState;

pub const MIN_VER: SemanticVersion = SemanticVersion::new(0, 1, 0);

bindgen!({
    path: "./wit/v0.1.0-draft",
    world: "nero:processor/processor",
    imports: { default: async | trappable },
    exports: { default: async },
    with: {
        "wasi:http": wasmtime_wasi_http::bindings::http,
        "wasi:logging": nero_wasi_logging::logging,
    },
});

pub fn linker(engine: &Engine) -> Result<Linker<WasmState>> {
    let mut linker = Linker::new(engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    wasmtime_wasi_http::add_only_http_to_linker_async(&mut linker)?;
    nero_wasi_logging::add_to_linker(&mut linker)?;
    nero_wit_process::add_to_linker(&mut linker, |s: &mut WasmState| {
        WitProcess::new(&s.wit_process_ctx, &mut s.table)
    })?;
    Ok(linker)
}

impl ffmpeg_sidecar::Host for WasmState {
    async fn ffmpeg_path(&mut self) -> wasmtime::Result<Option<String>> {
        todo!()
    }

    async fn ffprobe_path(&mut self) -> wasmtime::Result<Option<String>> {
        todo!()
    }
}
