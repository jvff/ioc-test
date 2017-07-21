use std::fmt::Display;
use std::io;

use tokio_service::NewService;

use super::expected_request::ExpectedRequest;
use super::mock_service::MockService;

#[derive(Clone)]
pub struct MockServiceFactory<A, B>
where
    A: Clone,
    B: Clone,
{
    expected_requests: Vec<ExpectedRequest<A, B>>,
}

impl<A, B> MockServiceFactory<A, B>
where
    A: Clone,
    B: Clone,
{
    pub fn new() -> Self {
        Self {
            expected_requests: Vec::new(),
        }
    }

    pub fn expect(&mut self, request: A, response: B) {
        let expected_request = ExpectedRequest { request, response };

        self.expected_requests.push(expected_request);
    }
}

impl<A, B> NewService for MockServiceFactory<A, B>
where
    A: Clone + Display + PartialEq,
    B: Clone,
{
    type Request = A;
    type Response = B;
    type Error = io::Error;
    type Instance = MockService<A, B>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let requests = self.expected_requests.clone();

        Ok(Self::Instance::with_expected_requests(requests))
    }
}
