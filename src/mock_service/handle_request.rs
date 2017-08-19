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
            self.reply_to_request(response.clone())
        } else {
            self.unexpected_request()
        }
    }

    fn reply_to_request(&self, response: B) -> Poll<B, Error> {
        Ok(Async::Ready(response))
    }

    fn unexpected_request(&self) -> Poll<B, Error> {
        Err(
            ErrorKind::UnexpectedRequest(self.request.to_string()).into(),
        )
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
