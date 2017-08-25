use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};

use super::verifiers::Verifier;

pub struct InstrumentedResponse<F, V, E>
where
    F: Future,
    V: Verifier<Response = F::Item>,
    E: From<F::Error>,
{
    response_future: F,
    verifier: Arc<Mutex<V>>,
    _error_type: PhantomData<E>,
}

impl<F, V, E> InstrumentedResponse<F, V, E>
where
    F: Future,
    V: Verifier<Response = F::Item>,
    E: From<F::Error>,
{
    pub fn new(response_future: F, verifier: Arc<Mutex<V>>) -> Self {
        Self {
            verifier,
            response_future,
            _error_type: PhantomData,
        }
    }
}

impl<F, V, E> Future for InstrumentedResponse<F, V, E>
where
    F: Future,
    V: Verifier<Response = F::Item>,
    E: From<F::Error>,
{
    type Item = <F as Future>::Item;
    type Error = E;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.response_future.poll()? {
            Async::Ready(response) => {
                let mut verifier = self.verifier.lock().expect(
                    "another thread panicked while holding a lock to a \
                     verifier",
                );

                verifier.response(&response);

                Ok(Async::Ready(response))
            }
            Async::NotReady => Ok(Async::NotReady),
        }
    }
}
