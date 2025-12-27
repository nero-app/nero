mod image;
mod video;

pub use image::*;
pub use video::*;

use bytes::Bytes;
use reqwest::Client;
use url::Url;

pub trait IntoReqwestRequest {
    fn into_reqwest_request(self, client: Client) -> Result<reqwest::Request, reqwest::Error>;
}

impl IntoReqwestRequest for http::Request<Option<Bytes>> {
    fn into_reqwest_request(self, client: Client) -> Result<reqwest::Request, reqwest::Error> {
        let (parts, body) = self.into_parts();

        let url = Url::parse(&parts.uri.to_string()).unwrap();

        let mut builder = client.request(parts.method, url);

        for (k, v) in parts.headers.iter() {
            builder = builder.header(k, v);
        }

        if let Some(body) = body {
            builder.body(body).build()
        } else {
            builder.build()
        }
    }
}
