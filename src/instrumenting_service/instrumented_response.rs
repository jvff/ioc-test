use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};

use super::verifiers::Verifier;

pub struct InstrumentedResponse<F, V>
where
    F: Future,
    V: Verifier<Response = F::Item>,
{
    response_future: F,
    verifier: Arc<Mutex<V>>,
}

impl<F, V> InstrumentedResponse<F, V>
where
    F: Future,
    V: Verifier<Response = F::Item>,
{
    pub fn new(response_future: F, verifier: Arc<Mutex<V>>) -> Self {
        Self {
            verifier,
            response_future,
        }
    }
}

impl<F, V> Future for InstrumentedResponse<F, V>
where
    F: Future,
    V: Verifier<Response = F::Item>,
{
    type Item = <F as Future>::Item;
    type Error = <F as Future>::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.response_future.poll() {
            Ok(Async::Ready(response)) => {
                let mut verifier = self.verifier.lock().expect(
                    "another thread panicked while holding a lock to a \
                     verifier",
                );

                verifier.response(&response);

                Ok(Async::Ready(response))
            }
            poll_result => poll_result,
        }
    }
}
