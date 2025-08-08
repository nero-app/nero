use axum::{
    body::Body,
    extract::{Query, State},
    http::{Request, Response},
    response::Result,
};
use reqwest::StatusCode;
use serde::Deserialize;
use url::Url;

use crate::{
    ProxyState,
    utils::{deserialize_base64_opt, deserialize_base64_plain_opt},
};

#[derive(Debug, Deserialize)]
pub struct TargetHttpResource {
    pub(crate) method: String,
    pub(crate) url: Url,
    #[serde(default, deserialize_with = "deserialize_base64_opt")]
    pub(crate) headers: Option<Vec<(String, String)>>,
    #[serde(default, deserialize_with = "deserialize_base64_plain_opt")]
    pub(crate) body: Option<String>,
}

pub async fn proxy_request(
    State(state): State<ProxyState>,
    Query(query): Query<TargetHttpResource>,
    req: Request<Body>,
) -> Result<Response<Body>, (StatusCode, String)> {
    todo!()
}
