use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use futures::{Async, Future, Poll};

use super::errors::{Error, ErrorKind};

pub struct HandleRequest<A, B> {
    request: A,
    expected_requests: Arc<Mutex<HashMap<A, B>>>,
}

impl<A, B> HandleRequest<A, B>
where
    A: Display + Eq + Hash,
    B: Clone,
{
    pub fn new(
        request: A,
        expected_requests: Arc<Mutex<HashMap<A, B>>>,
    ) -> Self {
        Self {
            request,
            expected_requests,
        }
    }

    fn handle_request(&self) -> Poll<B, Error> {
        let expected_requests = self.expected_requests.lock()?;

        if let Some(response) = expected_requests.get(&self.request) {
            Ok(Async::Ready(response.clone()))
        } else {
            let request = self.request.to_string();

            Err(ErrorKind::UnexpectedRequest(request).into())
        }
    }
}

impl<A, B> Future for HandleRequest<A, B>
where
    A: Display + Eq + Hash,
    B: Clone,
{
    type Item = B;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.handle_request().map_err(|error| error.into())
    }
}
