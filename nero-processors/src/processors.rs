pub mod since_v0_1_0_draft {
    use nero_runtime::semver::SemanticVersion;
    use wasmtime::component::bindgen;

    pub const MIN_VER: SemanticVersion = SemanticVersion::new(0, 1, 0);

    bindgen!({
        path: "./wit/v0.1.0-draft",
        world: "nero:processor/processor",
        async: true,
        with: {
            "wasi:clocks": wasmtime_wasi::p2::bindings::clocks,
            "wasi:io": wasmtime_wasi::p2::bindings::io,
            "wasi:http": wasmtime_wasi_http::bindings::http,
            "wasi:keyvalue": crate::keyvalue::keyvalue,
            "wasi:logging": nero_logging::logging,
        },
    });
}
