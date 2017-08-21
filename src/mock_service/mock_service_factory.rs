use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::sync::{Arc, Mutex};

use tokio_service::NewService;

use super::errors::Error;
use super::mock_service::MockService;

pub struct MockServiceFactory<A, B> {
    expected_requests: Arc<Mutex<HashMap<A, B>>>,
}

impl<A, B> MockServiceFactory<A, B>
where
    A: Clone + Eq + Hash,
{
    pub fn new(expected_requests: Arc<Mutex<HashMap<A, B>>>) -> Self {
        Self { expected_requests }
    }
}

impl<A, B> NewService for MockServiceFactory<A, B>
where
    A: Clone + Display + Eq + Hash,
    B: Clone,
{
    type Request = A;
    type Response = B;
    type Error = Error;
    type Instance = MockService<A, B>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let requests = self.expected_requests.clone();

        Ok(Self::Instance::new(requests))
    }
}
