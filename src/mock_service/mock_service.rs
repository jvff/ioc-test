use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::errors::Error;
use super::handle_request::HandleRequest;

pub struct MockService<A, B> {
    expected_requests: Arc<Mutex<HashMap<A, B>>>,
}

impl<A, B> MockService<A, B>
where
    A: Eq + Hash,
{
    pub fn new(expected_requests: Arc<Mutex<HashMap<A, B>>>) -> Self {
        Self { expected_requests }
    }
}

impl<A, B> Service for MockService<A, B>
where
    A: Clone + Display + Eq + Hash,
    B: Clone,
{
    type Request = A;
    type Response = B;
    type Error = Error;
    type Future = HandleRequest<A, B>;

    fn call(&self, request: Self::Request) -> Self::Future {
        HandleRequest::new(request, self.expected_requests.clone())
    }
}
