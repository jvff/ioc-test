use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll, Sink, Stream};

use super::wait_for_response::WaitForResponse;

pub struct FlushRequest<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    source: Arc<Mutex<I>>,
    sink: Arc<Mutex<O>>,
}

impl<I, O> FlushRequest<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    pub fn new(source: Arc<Mutex<I>>, sink: Arc<Mutex<O>>) -> Self {
        FlushRequest { source, sink }
    }
}

impl<I, O> Future for FlushRequest<I, O>
where
    I: Stream,
    O: Sink<SinkError = I::Error>,
{
    type Item = WaitForResponse<I>;
    type Error = I::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut sink = self.sink
            .lock()
            .expect("another thread panicked while holding a lock");

        match sink.poll_complete() {
            Ok(Async::Ready(())) => {
                Ok(Async::Ready(WaitForResponse::new(self.source.clone())))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err(error),
        }
    }
}
