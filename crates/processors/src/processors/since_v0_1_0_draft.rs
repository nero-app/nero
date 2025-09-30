use nero_runtime::semver::SemanticVersion;
use wasmtime::component::bindgen;

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
        "wasi:keyvalue": nero_wasi_keyvalue::keyvalue,
        "wasi:logging": nero_wasi_logging::logging,
    },
});

impl ffmpeg_sidecar::Host for WasmState {
    async fn ffmpeg_path(&mut self) -> wasmtime::Result<Option<String>> {
        todo!()
    }

    async fn ffprobe_path(&mut self) -> wasmtime::Result<Option<String>> {
        todo!()
    }
}
