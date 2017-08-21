use std::io;
use std::sync::{Arc, Mutex};

use futures::{Sink, Stream};
use tokio_service::NewService;

use super::errors::Error;
use super::proxy_service::ProxyService;

pub struct ProxyServiceFactory<I, O>
where
    I: Stream,
    O: Sink,
{
    source: Arc<Mutex<I>>,
    sink: Arc<Mutex<O>>,
}

impl<I, O> ProxyServiceFactory<I, O>
where
    I: Stream,
    O: Sink,
{
    pub fn new(source: Arc<Mutex<I>>, sink: Arc<Mutex<O>>) -> Self {
        Self { source, sink }
    }
}

impl<I, O> NewService for ProxyServiceFactory<I, O>
where
    I: Stream,
    O: Sink,
    I::Error: Into<Error>,
    O::SinkError: Into<Error>,
{
    type Request = O::SinkItem;
    type Response = I::Item;
    type Error = Error;
    type Instance = ProxyService<I, O>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(Self::Instance::new(self.source.clone(), self.sink.clone()))
    }
}
