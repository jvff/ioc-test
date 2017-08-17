use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::handle_request::HandleRequest;
use super::super::mock_server::FiniteService;

pub struct MockService<A, B> {
    expected_requests: Arc<Mutex<HashMap<A, B>>>,
    requests_to_verify: Arc<Mutex<HashSet<A>>>,
}

impl<A, B> MockService<A, B>
where
    A: Eq + Hash,
{
    pub fn new(
        expected_requests: Arc<Mutex<HashMap<A, B>>>,
        requests_to_verify: Arc<Mutex<HashSet<A>>>,
    ) -> Self {
        Self {
            requests_to_verify,
            expected_requests,
        }
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

impl<A, B> FiniteService for MockService<A, B>
where
    A: Clone + Display + Eq + Hash,
    B: Clone,
{
    fn has_finished(&self) -> io::Result<bool> {
        self.requests_to_verify
            .lock()
            .map(|requests_to_verify| requests_to_verify.is_empty())
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "failed to access requests to verify set",
                )
            })
    }
}
