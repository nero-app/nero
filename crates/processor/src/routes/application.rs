use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Response,
};
use uuid::Uuid;

use crate::{ServerState, error::Error};

#[allow(unused_variables)]
pub async fn handle_application_request(
    State(state): State<Arc<ServerState>>,
    Path(request_id): Path<Uuid>,
) -> Result<Response, Error> {
    todo!()
}
