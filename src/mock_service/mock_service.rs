use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::errors::Result;
use super::handle_request::HandleRequest;

pub struct MockService<A, B> {
    expected_requests: Arc<Mutex<HashMap<A, B>>>,
    requests_to_verify: Arc<Mutex<HashSet<A>>>,
}

impl<A, B> MockService<A, B>
where
    A: Eq + Hash,
{
    pub fn new(
        expected_requests: HashMap<A, B>,
        requests_to_verify: HashSet<A>,
    ) -> Self {
        Self {
            requests_to_verify: Arc::new(Mutex::new(requests_to_verify)),
            expected_requests: Arc::new(Mutex::new(expected_requests)),
        }
    }

    pub fn has_finished(&self) -> Result<bool> {
        Ok(self.requests_to_verify.lock()?.is_empty())
    }
}

impl<A, B> Service for MockService<A, B>
where
    A: Clone + Display + Eq + Hash,
    B: Clone,
{
    type Request = A;
    type Response = B;
    type Error = io::Error;
    type Future = HandleRequest<A, B>;

    fn call(&self, request: Self::Request) -> Self::Future {
        HandleRequest::new(
            request,
            self.expected_requests.clone(),
            self.requests_to_verify.clone(),
        )
    }
}
