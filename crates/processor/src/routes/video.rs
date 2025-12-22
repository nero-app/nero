use std::sync::Arc;

use axum::{
    extract::{Path, Request, State},
    response::Response,
};

use crate::{ServerState, error::Error};

#[allow(unused_variables)]
pub async fn handle_video_request(
    State(state): State<Arc<ServerState>>,
    Path(request_hash): Path<u64>,
    req: Request,
) -> Result<Response, Error> {
    todo!()
}
