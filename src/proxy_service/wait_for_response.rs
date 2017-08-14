use std::io;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll, Stream};

pub struct WaitForResponse<I>
where
    I: Stream,
{
    source: Arc<Mutex<I>>,
}

impl<I> WaitForResponse<I>
where
    I: Stream,
{
    pub fn new(source: Arc<Mutex<I>>) -> Self {
        WaitForResponse { source }
    }
}

impl<I> Future for WaitForResponse<I>
where
    I: Stream,
    <I as Stream>::Error: From<io::Error>,
{
    type Item = I::Item;
    type Error = I::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut source = self.source
            .lock()
            .expect("another thread panicked while holding a lock");

        match source.poll() {
            Ok(Async::Ready(Some(response))) => Ok(Async::Ready(response)),
            Ok(Async::Ready(None)) => Err(unexpected_eof().into()),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err(error),
        }
    }
}

fn unexpected_eof() -> io::Error {
    io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "stream terminated while waiting for response",
    )
}
