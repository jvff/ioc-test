use std::sync::{Arc, Mutex};

use futures::{Future, Poll, Sink, Stream};
use futures::future::Flatten;

use super::errors::Error;
use super::send_request::SendRequest;

pub struct ProxyRequest<I, O>
where
    I: Stream,
    O: Sink,
    I::Error: Into<Error>,
    O::SinkError: Into<Error>,
{
    future: Flatten<Flatten<SendRequest<I, O>>>,
}

impl<I, O> ProxyRequest<I, O>
where
    I: Stream,
    O: Sink,
    I::Error: Into<Error>,
    O::SinkError: Into<Error>,
{
    pub fn new(
        request: O::SinkItem,
        source: Arc<Mutex<I>>,
        sink: Arc<Mutex<O>>,
    ) -> Self {
        let send_request = SendRequest::new(request, source, sink);

        ProxyRequest {
            future: send_request.flatten().flatten(),
        }
    }
}

impl<I, O> Future for ProxyRequest<I, O>
where
    I: Stream,
    O: Sink,
    I::Error: Into<Error>,
    O::SinkError: Into<Error>,
{
    type Item = I::Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.future.poll()
    }
}
