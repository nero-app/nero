use anyhow::Result;
use wasmtime::component::HasData;

pub use self::generated::wasi::*;

mod generated {
    wasmtime::component::bindgen!({
        path: "wit",
        world: "wasi:logging/imports",
    });
}

pub struct WasiLogging;

impl logging::logging::Host for WasiLogging {
    fn log(&mut self, level: logging::logging::Level, context: String, message: String) {
        match level {
            logging::logging::Level::Trace => tracing::trace!(context = %context, "{message}"),
            logging::logging::Level::Debug => tracing::debug!(context = %context, "{message}"),
            logging::logging::Level::Info => tracing::info!(context = %context, "{message}"),
            logging::logging::Level::Warn => tracing::warn!(context = %context, "{message}"),
            logging::logging::Level::Error | logging::logging::Level::Critical => {
                tracing::error!(context = %context, "{message}")
            }
        }
    }
}

pub fn add_to_linker<T>(l: &mut wasmtime::component::Linker<T>) -> Result<()> {
    logging::logging::add_to_linker::<T, HasWasiLogging>(l, |_| WasiLogging)
}

struct HasWasiLogging;
impl HasData for HasWasiLogging {
    type Data<'a> = WasiLogging;
}
