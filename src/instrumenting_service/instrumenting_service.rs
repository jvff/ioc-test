use std::collections::HashSet;
use std::hash::Hash;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use tokio_service::Service;

pub struct InstrumentingService<T>
where
    T: Service,
    <T as Service>::Request: Eq + Hash,
{
    service: T,
    requests_to_verify: Arc<Mutex<HashSet<T::Request>>>,
}

impl<T> InstrumentingService<T>
where
    T: Service,
    <T as Service>::Request: Eq + Hash,
{
    pub fn new(service: T, requests_to_verify: HashSet<T::Request>) -> Self {
        Self {
            service,
            requests_to_verify: Arc::new(Mutex::new(requests_to_verify)),
        }
    }

    pub fn has_finished(
        &self,
    ) -> Result<bool, PoisonError<MutexGuard<HashSet<T::Request>>>> {
        Ok(self.requests_to_verify.lock()?.is_empty())
    }
}

impl<T> Service for InstrumentingService<T>
where
    T: Service,
    <T as Service>::Request: Eq + Hash,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn call(&self, request: Self::Request) -> Self::Future {
        let mut requests_to_verify = self.requests_to_verify.lock().expect(
            "another thread panicked while holding a lock for the list of \
             requests to verify",
        );

        requests_to_verify.remove(&request);

        self.service.call(request)
    }
}
