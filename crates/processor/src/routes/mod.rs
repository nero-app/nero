mod application;
mod image;
mod other;
mod video;

pub use application::*;
pub use image::*;
pub use other::*;
pub use video::*;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub(super) struct HandlersQueryParams {
    subtype: String,
}
