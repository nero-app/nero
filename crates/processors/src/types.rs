use std::fmt::Display;

use http_body_util::BodyExt;
use wasmtime_wasi_http::{bindings::http, types::HostOutgoingRequest};

pub trait AsyncTryFrom<T>: Sized {
    async fn async_try_from(value: T) -> anyhow::Result<Self>;
}

pub trait AsyncTryInto<T>: Sized {
    async fn async_try_into(self) -> anyhow::Result<T>;
}

impl<T, U> AsyncTryInto<U> for T
where
    U: AsyncTryFrom<T>,
{
    async fn async_try_into(self) -> anyhow::Result<U> {
        U::async_try_from(self).await
    }
}

#[derive(Clone, Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Other(String),
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Head => write!(f, "HEAD"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Connect => write!(f, "CONNECT"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Trace => write!(f, "TRACE"),
            Method::Patch => write!(f, "PATCH"),
            Method::Other(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Scheme {
    Http,
    Https,
    Other(String),
}

impl Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
            Scheme::Other(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OutgoingRequest {
    pub method: Method,
    pub scheme: Scheme,
    pub authority: Option<String>,
    pub path_with_query: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl AsyncTryFrom<HostOutgoingRequest> for OutgoingRequest {
    async fn async_try_from(req: HostOutgoingRequest) -> anyhow::Result<Self> {
        let method = match req.method {
            http::types::Method::Get => Method::Get,
            http::types::Method::Head => Method::Head,
            http::types::Method::Post => Method::Post,
            http::types::Method::Put => Method::Put,
            http::types::Method::Delete => Method::Delete,
            http::types::Method::Connect => Method::Connect,
            http::types::Method::Options => Method::Options,
            http::types::Method::Trace => Method::Trace,
            http::types::Method::Patch => Method::Patch,
            http::types::Method::Other(other) => Method::Other(other),
        };

        let scheme = match req.scheme.unwrap_or(http::types::Scheme::Https) {
            http::types::Scheme::Http => Scheme::Http,
            http::types::Scheme::Https => Scheme::Https,
            http::types::Scheme::Other(other) => Scheme::Other(other),
        };

        let headers = req
            .headers
            .iter()
            .map(|(name, value)| (name.to_string(), value.to_str().unwrap_or("").to_owned()))
            .collect();

        let body = match req.body {
            Some(box_body) => {
                let collected = box_body.collect().await?;
                Some(collected.to_bytes().to_vec())
            }
            None => None,
        };

        Ok(Self {
            method,
            scheme,
            authority: req.authority,
            path_with_query: req.path_with_query,
            headers,
            body,
        })
    }
}
