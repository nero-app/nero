use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Response,
};

use crate::{ServerState, error::Error};

#[allow(unused_variables)]
pub async fn handle_torrent_request(
    State(state): State<Arc<ServerState>>,
    Path(request_hash): Path<u64>,
) -> Result<Response, Error> {
    todo!()
}
