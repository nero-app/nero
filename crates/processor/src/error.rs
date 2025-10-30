use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid URL")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Request not found")]
    NotFound,

    #[error("Unsupported scheme")]
    UnsupportedScheme,

    #[error("Could not infer MIME type for request")]
    UnsupportedMediaType,

    #[error("Reqwest HTTP error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Remote server returned status {0}")]
    RemoteServer(StatusCode),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match &self {
            Error::InvalidUrl(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::UnsupportedScheme => StatusCode::BAD_REQUEST,
            Error::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Error::Reqwest(_) => {
                error!("Reqwest error: {:#}", self);
                StatusCode::BAD_GATEWAY
            }
            Error::RemoteServer(code) => {
                error!("Remote server returned status {}: {:#}", code, self);
                StatusCode::BAD_GATEWAY
            }
            Error::Io(_) => {
                error!("IO error: {:#}", self);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, self.to_string()).into_response()
    }
}
