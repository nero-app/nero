mod proxy;
mod resolver;

use tauri::{
    Runtime,
    plugin::{Builder as PluginBuilder, TauriPlugin},
};

pub fn init<R: Runtime>(host: String, port: u16) -> TauriPlugin<R> {
    PluginBuilder::new("http-resources")
        .setup(move |_app, _api| {
            tauri::async_runtime::spawn(proxy::init(host, port));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![resolver::resolve_resource])
        .build()
}
