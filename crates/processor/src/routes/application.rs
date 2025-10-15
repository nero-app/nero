use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::Response,
};
use uuid::Uuid;

use crate::{ServerState, error::Error, routes::HandlersQueryParams};

#[allow(unused_variables)]
pub async fn handle_application_request(
    State(state): State<Arc<ServerState>>,
    Path(request_id): Path<Uuid>,
    Query(query): Query<HandlersQueryParams>,
) -> Result<Response, Error> {
    todo!()
}
