use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::instrumented_response::InstrumentedResponse;
use super::super::async_server::FiniteService;
use super::verifiers::Verifier;

pub struct InstrumentingService<S, V>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
{
    service: S,
    verifier: Arc<Mutex<V>>,
}

impl<S, V> InstrumentingService<S, V>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
{
    pub fn new(service: S, verifier: V) -> Self {
        Self {
            service,
            verifier: Arc::new(Mutex::new(verifier)),
        }
    }
}

impl<S, V> Service for InstrumentingService<S, V>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = InstrumentedResponse<S::Future, V>;

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

impl<S, V> FiniteService for InstrumentingService<S, V>
where
    S: Service,
    V: Verifier<Request = S::Request, Response = S::Response>,
    S::Error: From<V::Error>,
{
    fn has_finished(&self) -> Result<bool, S::Error> {
        let verifier = self.verifier.lock().expect(
            "another thread panicked while holding a lock to a verifier",
        );

        Ok(verifier.has_finished()?)
    }
}
