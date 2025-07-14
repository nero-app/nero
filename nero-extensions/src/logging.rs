use wasmtime::component::HasData;
use wasmtime_wasi::{ResourceTable, p2::IoView};

wasmtime::component::bindgen!({
    path: "./wit/v0.0.1/deps/logging",
});

pub use wasi::logging::logging::{self, add_to_linker};

pub struct WasiLogging<T>(T);
impl<T: 'static> HasData for WasiLogging<T> {
    type Data<'a> = WasiLoggingImpl<&'a mut T>;
}

pub struct WasiLoggingImpl<T>(pub wasmtime_wasi::p2::IoImpl<T>);

impl<T: IoView> IoView for WasiLoggingImpl<T> {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(&mut self.0.0)
    }
}

impl<T> logging::Host for WasiLoggingImpl<T> {
    fn log(&mut self, level: logging::Level, context: String, message: String) {
        match level {
            logging::Level::Trace => tracing::trace!(context = %context, "{message}"),
            logging::Level::Debug => tracing::debug!(context = %context, "{message}"),
            logging::Level::Info => tracing::info!(context = %context, "{message}"),
            logging::Level::Warn => tracing::warn!(context = %context, "{message}"),
            logging::Level::Error | logging::Level::Critical => {
                tracing::error!(context = %context, "{message}")
            }
        }
    }
}
