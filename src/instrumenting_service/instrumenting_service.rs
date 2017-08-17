use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::instrumented_response::InstrumentedResponse;
use super::super::async_server::FiniteService;
use super::verifier::Verifier;

pub struct InstrumentingService<T, V>
where
    T: Service,
    V: Verifier<Request = T::Request, Response = T::Response>,
{
    service: T,
    verifier: Arc<Mutex<V>>,
}

impl<T, V> InstrumentingService<T, V>
where
    T: Service,
    V: Verifier<Request = T::Request, Response = T::Response>,
{
    pub fn new(service: T, verifier: V) -> Self {
        Self {
            service,
            verifier: Arc::new(Mutex::new(verifier)),
        }
    }
}

impl<T, V> Service for InstrumentingService<T, V>
where
    T: Service,
    V: Verifier<Request = T::Request, Response = T::Response>,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = T::Error;
    type Future = InstrumentedResponse<T::Future, V>;

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

impl<T, V> FiniteService for InstrumentingService<T, V>
where
    T: Service,
    V: Verifier<Request = T::Request, Response = T::Response>,
    T::Error: From<V::Error>,
{
    fn has_finished(&self) -> Result<bool, T::Error> {
        let verifier = self.verifier.lock().expect(
            "another thread panicked while holding a lock to a verifier",
        );

        Ok(verifier.has_finished()?)
    }
}
