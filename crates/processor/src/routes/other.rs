use std::sync::Arc;

use axum::{
    extract::{Path, Request, State},
    response::Response,
};
use uuid::Uuid;

use crate::{ServerState, error::Error};

#[allow(unused_variables)]
pub async fn handle_other_request(
    State(state): State<Arc<ServerState>>,
    Path(request_id): Path<Uuid>,
    req: Request,
) -> Result<Response, Error> {
    todo!()
}
