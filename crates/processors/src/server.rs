use std::{net::SocketAddr, sync::Arc};

use anyhow::{Result, anyhow, bail};
use bytes::Bytes;
use http_body_util::{BodyExt, Full, combinators::BoxBody};
use hyper::server::conn::http1;
use moka::future::Cache;
use tokio::net::{TcpListener, TcpStream};
use url::Url;
use uuid::Uuid;
use wasmtime::Store;
use wasmtime_wasi_http::{
    WasiHttpView, bindings::http::types::Scheme, body::HyperOutgoingBody, io::TokioIo,
    types::HostOutgoingRequest,
};

use crate::{
    ProcessorPre, WasmProcessor, WasmState,
    types::{AsyncTryInto, OutgoingRequest},
};

type TargetBody = BoxBody<Bytes, hyper::Error>;

pub struct HttpServer {
    addr: SocketAddr,
    processor_pre: ProcessorPre,
    outgoing_requests: Cache<Uuid, OutgoingRequest>,
}

impl HttpServer {
    pub fn new(addr: SocketAddr, processor: WasmProcessor) -> Arc<Self> {
        Arc::new(Self {
            addr,
            processor_pre: processor.processor_pre,
            // TODO: configure time_to_idle - time_to_live
            outgoing_requests: Cache::builder().build(),
        })
    }

    pub async fn run(self: Arc<Self>) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            let (client, addr) = listener.accept().await?;
            let server = self.clone();
            tokio::task::spawn(server.handle_client(client, addr));
        }
    }

    async fn handle_client(self: Arc<Self>, client: TcpStream, addr: std::net::SocketAddr) {
        let connection = TokioIo::new(client);
        let service = hyper::service::service_fn(move |req| {
            let server = self.clone();
            async move { server.handle_request(req).await }
        });

        if let Err(e) = http1::Builder::new()
            .keep_alive(true)
            .serve_connection(connection, service)
            .await
        {
            eprintln!("error serving client[{addr}]: {e:?}");
        }
    }

    pub async fn store_outgoing_request(&self, request: HostOutgoingRequest) -> Result<Url> {
        let id = Uuid::new_v4();
        let outgoing_request = request.async_try_into().await?;
        self.outgoing_requests.insert(id, outgoing_request).await;

        let url = Url::parse(&format!("http://{}/{id}", self.addr))?;
        Ok(url)
    }

    async fn handle_request(
        self: Arc<Self>,
        req: hyper::Request<hyper::body::Incoming>,
    ) -> Result<hyper::Response<HyperOutgoingBody>> {
        let path = req.uri().path().trim_start_matches('/');
        let uuid = Uuid::parse_str(path)?;

        let outgoing_request = self
            .outgoing_requests
            .get(&uuid)
            .await
            .ok_or_else(|| anyhow!("outgoing request not found for request: {:?}", req))?;

        let req = self.build_target_request(
            req.method(),
            &format!("http://{}/{}", self.addr, uuid),
            &outgoing_request,
        )?;
        self.forward_to_wasm(req).await
    }

    fn build_target_request(
        &self,
        method: &hyper::Method,
        uri: &str,
        outgoing_request: &OutgoingRequest,
    ) -> Result<hyper::Request<TargetBody>> {
        let mut builder = hyper::Request::builder()
            .method(method)
            .uri(uri)
            .header("X-Target-Method", outgoing_request.method.to_string())
            .header("X-Target-Scheme", outgoing_request.scheme.to_string());

        if let Some(authority) = &outgoing_request.authority {
            builder = builder.header("X-Target-Authority", authority);
        }
        if let Some(path) = &outgoing_request.path_with_query {
            builder = builder.header("X-Target-Path", path);
        }
        for (name, value) in &outgoing_request.headers {
            let header_name = format!("X-Target-Header-{}", name.as_str());
            builder = builder.header(&header_name, value);
        }

        let body = outgoing_request
            .body
            .as_ref()
            .map(|b| Full::new(Bytes::from(b.clone())))
            .unwrap_or_else(|| Full::new(Bytes::new()))
            .map_err(|_| unreachable!())
            .boxed();

        let req = builder.body(body)?;
        Ok(req)
    }

    async fn forward_to_wasm(
        &self,
        req: hyper::Request<TargetBody>,
    ) -> Result<hyper::Response<HyperOutgoingBody>> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let engine = self.processor_pre.engine();
        let mut store = Store::new(engine, WasmState::default());

        let incoming_req = store.data_mut().new_incoming_request(Scheme::Http, req)?;
        let response_outparam = store.data_mut().new_response_outparam(sender)?;

        let processor_pre = self.processor_pre.clone();
        let task = tokio::task::spawn(async move {
            let processor = processor_pre.instantiate_async(&mut store).await?;
            processor
                .handle_incoming_request(&mut store, incoming_req, response_outparam)
                .await
        });

        match receiver.await {
            Ok(Ok(resp)) => Ok(resp),
            Ok(Err(e)) => Err(e.into()),
            Err(_) => {
                let e = match task.await {
                    Ok(Ok(())) => bail!("guest never invoked `response-outparam::set` method"),
                    Ok(Err(e)) => e,
                    Err(e) => e.into(),
                };
                Err(e.context("guest never invoked `response-outparam::set` method"))
            }
        }
    }
}
