use std::fmt::Debug;

use super::verifiers;
use super::verifiers::{Verifier, VerifierFactory, VerifyRequest,
                       VerifyRequestResponse};

pub enum WhenVerifier<A, B> {
    Request(VerifyRequest<A, B>),
    RequestResponse(VerifyRequestResponse<A, B>),
}

impl<A, B> WhenVerifier<A, B> {
    pub fn for_request(request: A) -> Self {
        WhenVerifier::Request(VerifyRequest::new(request))
    }

    pub fn for_request_response(request: A, response: B) -> Self {
        WhenVerifier::RequestResponse(
            VerifyRequestResponse::new(request, response),
        )
    }
}

impl<A, B> Verifier for WhenVerifier<A, B>
where
    A: Debug + Eq,
    B: Debug + Eq,
{
    type Request = A;
    type Response = B;
    type Error = verifiers::Error;

    fn request(&mut self, request: &Self::Request) {
        match *self {
            WhenVerifier::Request(ref mut verifier) => {
                verifier.request(request)
            }
            WhenVerifier::RequestResponse(ref mut verifier) => {
                verifier.request(request)
            }
        }
    }

    fn response(&mut self, response: &Self::Response) {
        match *self {
            WhenVerifier::Request(ref mut verifier) => {
                verifier.response(response)
            }
            WhenVerifier::RequestResponse(ref mut verifier) => {
                verifier.response(response)
            }
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        match *self {
            WhenVerifier::Request(ref verifier) => verifier.has_finished(),
            WhenVerifier::RequestResponse(ref verifier) => {
                verifier.has_finished()
            }
        }
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        match *self {
            WhenVerifier::Request(ref mut verifier) => verifier.force_stop(),
            WhenVerifier::RequestResponse(ref mut verifier) => {
                verifier.force_stop()
            }
        }
    }
}

impl<A, B> VerifierFactory for WhenVerifier<A, B>
where
    A: Clone + Debug + Eq,
    B: Clone + Debug + Eq,
{
    type Verifier = WhenVerifier<A, B>;

    fn create(&self) -> Self::Verifier {
        match *self {
            WhenVerifier::Request(ref verifier) => WhenVerifier::Request(
                verifier.create(),
            ),
            WhenVerifier::RequestResponse(ref verifier) => {
                WhenVerifier::RequestResponse(verifier.create())
            }
        }
    }
}
