use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;
use std::io;

use tokio_service::NewService;

use super::mock_service::MockService;
use super::when::When;

macro_rules! request_response_map {
    ( $object:expr , $($request:expr => $response:expr),+ $(,)* ) => {
        $($object.when($request).reply_with($response));+
    }
}

pub struct MockServiceFactory<A, B> {
    expected_requests: HashMap<A, B>,
    requests_to_verify: HashSet<A>,
}

impl<A, B> MockServiceFactory<A, B>
where
    A: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            expected_requests: HashMap::new(),
            requests_to_verify: HashSet::new(),
        }
    }

    pub fn when<C>(&mut self, request: C) -> When<A, B>
    where
        C: Into<A>,
    {
        When::new(self, request.into())
    }

    pub fn expect(&mut self, request: A, response: B) -> &mut Self {
        self.expected_requests.insert(request, response);

        self
    }

    pub fn verify<C>(&mut self, request: C) -> &mut Self
    where
        C: Into<A>,
    {
        self.requests_to_verify.insert(request.into());

        self
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
