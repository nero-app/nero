mod m3u8;
mod utils;

use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, HeaderName, HeaderValue, Response},
    response::IntoResponse,
    routing::get,
};
use reqwest::{Client, StatusCode};
use tauri::{
    Runtime,
    plugin::{Builder as PluginBuilder, TauriPlugin},
};
use tokio::net::TcpListener;
use url::Url;

use crate::{
    m3u8::{is_m3u8_content, rewrite_m3u8},
    utils::{extract_headers, extract_target_url},
};

#[derive(Clone)]
pub struct ProxyState {
    host: String,
    port: u16,
    http_client: Arc<Client>,
}

// TODO: Improve error handling

async fn proxy_handler(state: State<ProxyState>, req: Request<Body>) -> impl IntoResponse {
    let full_url = format!("http://{}:{}{}", state.host, state.port, req.uri());
    let Ok(url) = Url::parse(&full_url) else {
        return (StatusCode::BAD_REQUEST, "Invalid URL").into_response();
    };
    let Some(target_url) = extract_target_url(&url) else {
        return (StatusCode::BAD_REQUEST, "Missing target parameter").into_response();
    };
    let headers = extract_headers(&url);
    match proxy_request(&state, &target_url, headers, req).await {
        Ok(resp) => resp.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

async fn proxy_request(
    state: &ProxyState,
    target_url: &str,
    custom_headers: HashMap<String, String>,
    req: Request<Body>,
) -> Result<Response<Body>, Box<dyn std::error::Error>> {
    let url = Url::parse(target_url)?;

    let mut req_headers = HeaderMap::new();
    for (k, v) in req.headers() {
        if k == "host" || k == "accept-encoding" {
            continue;
        }
        req_headers.insert(k.clone(), v.clone());
    }
    for (k, v) in custom_headers {
        req_headers.insert(HeaderName::from_str(&k)?, HeaderValue::from_str(&v)?);
    }
    let res = state
        .http_client
        .get(target_url)
        .headers(req_headers)
        .send()
        .await?;

    let mut headers = HeaderMap::new();
    for (k, v) in res.headers() {
        if k == "content-length" {
            continue;
        }
        // Ignore forbidden headers
        if k == "connection" || k == "trailer" || k == "transfer-encoding" || k == "upgrade" {
            continue;
        }
        headers.insert(k, v.clone());
    }

    let status_code = res.status();

    let has_range_header = req.headers().get("range").is_some();
    let should_rewrite_m3u8 = is_m3u8_content(&url, &headers) && !has_range_header;
    let body = match should_rewrite_m3u8 {
        true => {
            let proxy_url = Url::parse(&format!("http://{}:{}", state.host, state.port))?;
            let bytes = res.bytes().await?;
            let rewritten = rewrite_m3u8(&proxy_url, &url.join("./")?, &bytes);

            headers.insert("content-length", HeaderValue::from(rewritten.len()));
            Body::from(rewritten)
        }
        false => Body::from_stream(res.bytes_stream()),
    };

    let mut builder = Response::builder().status(status_code);
    for (k, v) in headers.iter() {
        builder = builder.header(k, v);
    }
    let response = builder.body(body)?;
    Ok(response)
}

pub async fn init(host: String, port: u16) {
    let http_client = Client::new();
    let state = ProxyState {
        host: host.clone(),
        port,
        http_client: Arc::new(http_client),
    };

    let app = Router::new()
        .route("/", get(proxy_handler))
        .with_state(state);

    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}

// TODO: Add tauri command to get the proxy URL from the frontend

pub struct Builder {
    host: String,
    port: u16,
}

impl Builder {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        PluginBuilder::new("tauri-plugin-video-proxy")
            .setup(move |_app, _api| {
                tauri::async_runtime::spawn(init(self.host, self.port));
                Ok(())
            })
            .build()
    }
}
