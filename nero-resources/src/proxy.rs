use axum::{Router, body::Body, http::Response, routing::get};
use reqwest::StatusCode;
use tokio::net::TcpListener;

pub async fn init(host: String, port: u16) {
    let app = Router::new().route("/", get(proxy_request));

    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap()
}

pub async fn proxy_request() -> Result<Response<Body>, (StatusCode, String)> {
    todo!()
}
