use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, Request, State},
    response::Response,
};
use uuid::Uuid;

use crate::{ServerState, error::Error, routes::ForwardRequest};

pub async fn handle_image_request(
    State(state): State<Arc<ServerState>>,
    Path(request_id): Path<Uuid>,
    incoming_request: Request<Body>,
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

    ForwardRequest::new(state.http_client.clone(), uri, headers)
        .send()
        .await
}
