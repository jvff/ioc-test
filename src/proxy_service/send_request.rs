use std::sync::{Arc, Mutex};

use futures::{Async, AsyncSink, Future, Poll, Sink, Stream};

use super::flush_request::FlushRequest;

pub struct SendRequest<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    request: Option<O::SinkItem>,
    source: Arc<Mutex<I>>,
    sink: Arc<Mutex<O>>,
}

impl<I, O> SendRequest<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    pub fn new(
        request: O::SinkItem,
        source: Arc<Mutex<I>>,
        sink: Arc<Mutex<O>>,
    ) -> Self {
        SendRequest {
            source,
            sink,
            request: Some(request),
        }
    }
}

impl<I, O> Future for SendRequest<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    type Item = FlushRequest<I, O>;
    type Error = I::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let request = self.request
            .take()
            .expect("can't send a request twice using the same future");

        let mut sink = self.sink
            .lock()
            .expect("another thread panicked while holding a lock");

        match sink.start_send(request) {
            Ok(AsyncSink::Ready) => {
                let source = self.source.clone();
                let sink = self.sink.clone();

                Ok(Async::Ready(FlushRequest::new(source, sink)))
            }
            Ok(AsyncSink::NotReady(item)) => {
                self.request = Some(item);
                Ok(Async::NotReady)
            }
            Err(error) => Err(error),
        }
    }
}
