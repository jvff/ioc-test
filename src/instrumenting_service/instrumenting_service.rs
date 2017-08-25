use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::instrumented_response::InstrumentedResponse;
use super::super::async_server::FiniteService;
use super::verifiers::Verifier;

pub struct InstrumentingService<S, V, E>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
    E: From<S::Error> + From<V::Error>,
{
    service: S,
    verifier: Arc<Mutex<V>>,
    _error_type: PhantomData<E>,
}

impl<S, V, E> InstrumentingService<S, V, E>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
    E: From<S::Error> + From<V::Error>,
{
    pub fn new(service: S, verifier: V) -> Self {
        Self {
            service,
            verifier: Arc::new(Mutex::new(verifier)),
            _error_type: PhantomData,
        }
    }
}

impl<S, V, E> Service for InstrumentingService<S, V, E>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
    E: From<S::Error> + From<V::Error>,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = E;
    type Future = InstrumentedResponse<S::Future, V, E>;

    fn call(&self, request: Self::Request) -> Self::Future {
        let mut verifier = self.verifier.lock().expect(
            "another thread panicked while holding a lock to a verifier",
        );

        verifier.request(&request);

        InstrumentedResponse::new(
            self.service.call(request),
            self.verifier.clone(),
        )
    }
}

impl<S, V, E> FiniteService for InstrumentingService<S, V, E>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
    E: From<S::Error> + From<V::Error>,
{
    fn has_finished(&self) -> Result<bool, E> {
        let verifier = self.verifier.lock().expect(
            "another thread panicked while holding a lock to a verifier",
        );

        Ok(verifier.has_finished()?)
    }
}
