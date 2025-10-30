use std::sync::Arc;

use axum::{
    extract::{Path, Request, State},
    response::Response,
};
use uuid::Uuid;

use crate::{ServerState, error::Error, routes::ForwardRequest};

pub async fn handle_other_request(
    State(state): State<Arc<ServerState>>,
    Path(request_id): Path<Uuid>,
    incoming_request: Request,
) -> Result<Response, Error> {
    let stored_request = state
        .http_requests
        .get(&request_id)
        .await
        .ok_or(Error::NotFound)?;

    let uri = stored_request.uri().to_string();

    let mut headers = stored_request.headers().clone();
    for (name, value) in incoming_request.headers().iter() {
        headers.insert(name.clone(), value.clone());
    }

    let mut forward = ForwardRequest::new(state.http_client.clone(), uri, headers);
    if let Some(body) = stored_request.into_body() {
        forward = forward.body(body);
    }
    forward.send().await
}
