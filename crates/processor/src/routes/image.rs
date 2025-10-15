use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::{IntoResponse, Redirect, Response},
};
use tracing::warn;
use uuid::Uuid;

use crate::{ServerState, error::Error};

pub async fn handle_image_request(
    State(state): State<Arc<ServerState>>,
    Path(request_id): Path<Uuid>,
    req: Request<Body>,
) -> Result<Response, Error> {
    let request = state
        .http_requests
        .get(&request_id)
        .await
        .ok_or(Error::NotFound)?;

    if request.headers().is_empty() {
        let redirect = Redirect::permanent(&request.uri().to_string());
        return Ok(redirect.into_response());
    }

    let mut req = state
        .http_client
        .get(request.uri().to_string())
        .headers(req.headers().clone());
    for (k, v) in request.headers().iter() {
        req = req.header(k, v);
    }

    let res = req.send().await?;

    let status = res.status();
    if !status.is_success() {
        warn!(
            "Remote server returned status {} for {}",
            status,
            request.uri()
        );
        return Err(Error::RemoteServer(status));
    }

    let headers = res.headers().clone();
    let stream = res.bytes_stream();

    let body = Body::from_stream(stream);
    let mut response = Response::new(body);
    *response.status_mut() = status;
    *response.headers_mut() = headers;

    Ok(response)
}
