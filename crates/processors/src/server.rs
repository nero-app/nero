use std::{net::SocketAddr, sync::Arc};

use anyhow::{Result, bail};
use hyper::server::conn::http1;
use tokio::{net::TcpListener, sync::OnceCell};
use wasmtime::Store;
use wasmtime_wasi_http::{
    WasiHttpView, bindings::http::types::Scheme, body::HyperOutgoingBody, io::TokioIo,
};

use crate::{ProcessorPre, WasmProcessor, WasmState};

pub static SERVER_ADDR: OnceCell<SocketAddr> = OnceCell::const_new();

pub struct HttpServer {
    addr: SocketAddr,
    processor_pre: ProcessorPre,
}

impl HttpServer {
    pub fn new(addr: SocketAddr, processor: &WasmProcessor) -> Result<Arc<Self>> {
        SERVER_ADDR.set(addr)?;

        Ok(Arc::new(Self {
            addr,
            processor_pre: processor.processor_pre.clone(),
        }))
    }

    pub async fn run(self: Arc<Self>) -> Result<()> {
        let listener = TcpListener::bind(self.addr).await?;
        loop {
            let (client, addr) = listener.accept().await?;
            let server = self.clone();
            tokio::task::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .serve_connection(
                        TokioIo::new(client),
                        hyper::service::service_fn(move |req| {
                            let server = server.clone();
                            async move { server.handle_request(req).await }
                        }),
                    )
                    .await
                {
                    eprintln!("error serving client[{addr}]: {e:?}");
                }
            });
        }
    }

    async fn handle_request(
        self: Arc<Self>,
        req: hyper::Request<hyper::body::Incoming>,
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
                    Ok(Ok(())) => {
                        bail!("guest never invoked `response-outparam::set` method")
                    }
                    Ok(Err(e)) => e,
                    Err(e) => e.into(),
                };
                Err(e.context("guest never invoked `response-outparam::set` method"))
            }
        }
    }
}
