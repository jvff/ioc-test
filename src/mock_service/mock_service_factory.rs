use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::io;
use std::sync::{Arc, Mutex};

use tokio_service::NewService;

use super::mock_service::MockService;

macro_rules! request_response_map {
    ( $object:expr , $($request:expr => $response:expr),+ $(,)* ) => {
        $($object.when($request).reply_with($response));+
    }
}

pub struct MockServiceFactory<A, B> {
    expected_requests: Arc<Mutex<HashMap<A, B>>>,
    requests_to_verify: Arc<Mutex<HashSet<A>>>,
}

impl<A, B> MockServiceFactory<A, B>
where
    A: Clone + Eq + Hash,
{
    pub fn new(
        expected_requests: Arc<Mutex<HashMap<A, B>>>,
        requests_to_verify: Arc<Mutex<HashSet<A>>>,
    ) -> Self {
        Self {
            expected_requests,
            requests_to_verify,
        }
    }
}

impl<A, B> NewService for MockServiceFactory<A, B>
where
    A: Clone + Display + Eq + Hash,
    B: Clone,
{
    type Request = A;
    type Response = B;
    type Error = io::Error;
    type Instance = MockService<A, B>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let requests = self.expected_requests.clone();
        let requests_to_verify = self.requests_to_verify.clone();

        Ok(Self::Instance::new(requests, requests_to_verify))
    }
}
