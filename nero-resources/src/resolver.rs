use nero_types::types::HttpResource;
use url::Url;

#[tauri::command]
pub async fn resolve_resource(resource: HttpResource) -> Url {
    todo!("{:?}", resource)
}
