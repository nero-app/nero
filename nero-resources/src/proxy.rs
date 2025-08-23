use axum::{
    Router,
    body::Body,
    extract::{Query, Request, State},
    http::Response,
    routing::get,
};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use tokio::net::TcpListener;
use url::Url;

use crate::utils::{deserialize_base64_opt, deserialize_base64_plain_opt};

#[derive(Clone)]
pub struct ProxyState {
    host: String,
    port: u16,
    http_client: Client,
}

pub async fn init(host: String, port: u16) {
    let http_client = Client::new();
    let state = ProxyState {
        host: host.clone(),
        port,
        http_client,
    };

    let app = Router::new()
        .route("/", get(proxy_request))
        .with_state(state);

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}

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
