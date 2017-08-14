use std::io;
use std::sync::{Arc, Mutex};

use futures::{Sink, Stream};
use tokio_service::Service;

use super::proxy_request::ProxyRequest;

pub struct ProxyService<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    source: Arc<Mutex<I>>,
    sink: Arc<Mutex<O>>,
}

impl<I, O> ProxyService<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    pub fn new(source: I, sink: O) -> Self {
        ProxyService {
            source: Arc::new(Mutex::new(source)),
            sink: Arc::new(Mutex::new(sink)),
        }
    }
}

impl<I, O> Service for ProxyService<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
    <I as Stream>::Error: From<io::Error>,
{
    type Request = O::SinkItem;
    type Response = I::Item;
    type Error = I::Error;
    type Future = ProxyRequest<I, O>;

    fn call(&self, request: Self::Request) -> Self::Future {
        ProxyRequest::new(request, self.source.clone(), self.sink.clone())
    }
}
