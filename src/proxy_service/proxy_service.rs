use std::sync::{Arc, Mutex};

use futures::{Sink, Stream};
use tokio_service::Service;

use super::errors::Error;
use super::proxy_request::ProxyRequest;

pub struct ProxyService<I, O>
where
    I: Stream,
    O: Sink,
{
    source: Arc<Mutex<I>>,
    sink: Arc<Mutex<O>>,
}

impl<I, O> ProxyService<I, O>
where
    I: Stream,
    O: Sink,
{
    pub fn new(source: Arc<Mutex<I>>, sink: Arc<Mutex<O>>) -> Self {
        ProxyService { source, sink }
    }
}

impl<I, O> Service for ProxyService<I, O>
where
    I: Stream,
    O: Sink,
    I::Error: Into<Error>,
    O::SinkError: Into<Error>,
{
    type Request = O::SinkItem;
    type Response = I::Item;
    type Error = Error;
    type Future = ProxyRequest<I, O>;

    fn call(&self, request: Self::Request) -> Self::Future {
        ProxyRequest::new(request, self.source.clone(), self.sink.clone())
    }
}
