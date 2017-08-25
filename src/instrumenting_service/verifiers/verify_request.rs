use std::fmt::Debug;
use std::marker::PhantomData;

use super::errors::{Error, ErrorKind};
use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

#[derive(Clone)]
pub struct VerifyRequest<A, B> {
    verified: bool,
    request: A,
    _response: PhantomData<B>,
}

impl<A, B> VerifyRequest<A, B> {
    pub fn new(request: A) -> Self {
        Self {
            request,
            verified: false,
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
        if !self.verified {
            self.verified = self.request == *request;
        }
    }

    fn response(&mut self, _response: &Self::Response) {}

    fn has_finished(&self) -> Result<bool, Self::Error> {
        Ok(self.verified)
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        if self.verified {
            Ok(())
        } else {
            let request = format!("{:?}", self.request);

            Err(ErrorKind::RequestWasntVerified(request).into())
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
