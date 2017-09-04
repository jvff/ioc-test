use std::fmt::Debug;
use std::marker::PhantomData;

use super::errors::{Error, ErrorKind, Result};
use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

#[derive(Clone)]
pub struct VerifyRequest<A, B> {
    request: A,
    status: Status,
    _response: PhantomData<B>,
}

#[derive(Clone, PartialEq)]
pub enum Status {
    Active,
    Verified,
    IncorrectRequest(String),
}

impl<A, B> VerifyRequest<A, B> {
    pub fn new(request: A) -> Self {
        Self {
            request,
            status: Status::Active,
            _response: PhantomData,
        }
    }
}

impl<A, B> Verifier for VerifyRequest<A, B>
where
    A: Debug + Eq,
{
    type Request = A;
    type Response = B;
    type Error = Error;

    fn request(&mut self, request: &Self::Request) {
        if self.status == Status::Active {
            if self.request == *request {
                self.status = Status::Verified;
            } else {
                let received_request = format!("{:?}", *request);

                self.status = Status::IncorrectRequest(received_request);
            }
        }
    }

    fn response(&mut self, _response: &Self::Response) {}

    fn has_finished(&self) -> Result<bool> {
        match self.status {
            Status::Active => Ok(false),
            Status::Verified => Ok(true),
            Status::IncorrectRequest(ref received_request) => {
                let expected_request = format!("{:?}", self.request);

                Err(
                    ErrorKind::IncorrectRequest(
                        received_request.clone(),
                        expected_request,
                    ).into(),
                )
            }
        }
    }

    fn force_stop(&mut self) -> Result<()> {
        match self.status {
            Status::Verified => Ok(()),
            Status::Active => {
                let request = format!("{:?}", self.request);

                Err(ErrorKind::RequestWasntVerified(request).into())
            }
            Status::IncorrectRequest(ref received_request) => {
                let expected_request = format!("{:?}", self.request);

                Err(
                    ErrorKind::IncorrectRequest(
                        received_request.clone(),
                        expected_request,
                    ).into(),
                )
            }
        }
    }
}

impl<A, B> VerifierFactory for VerifyRequest<A, B>
where
    A: Clone + Debug + Eq,
{
    type Verifier = Self;

    fn create(&self) -> Self::Verifier {
        VerifyRequest::new(self.request.clone())
    }
}
