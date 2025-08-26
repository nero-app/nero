const COMMANDS: &[&str] = &["resolve_resource"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
