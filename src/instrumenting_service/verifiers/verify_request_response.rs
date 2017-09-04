use std::fmt::Debug;

use super::errors::{Error, ErrorKind};
use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

#[derive(Clone, Eq, PartialEq)]
enum Status {
    WaitingForRequest,
    RequestVerified,
    Verified,
    IncorrectRequest(String),
    IncorrectResponse(String),
}

#[derive(Clone)]
pub struct VerifyRequestResponse<A, B> {
    request: A,
    response: B,
    status: Status,
}

impl<A, B> VerifyRequestResponse<A, B> {
    pub fn new(request: A, response: B) -> Self {
        Self {
            request,
            response,
            status: Status::WaitingForRequest,
        }
    }
}

impl<A, B> Verifier for VerifyRequestResponse<A, B>
where
    A: Debug + Eq,
    B: Debug + Eq,
{
    type Request = A;
    type Response = B;
    type Error = Error;

    fn request(&mut self, request: &Self::Request) {
        if self.status == Status::WaitingForRequest {
            if self.request == *request {
                self.status = Status::RequestVerified;
            } else {
                let received_request = format!("{:?}", *request);

                self.status = Status::IncorrectRequest(received_request);
            }
        }
    }

    fn response(&mut self, response: &Self::Response) {
        if self.status == Status::RequestVerified {
            if self.response == *response {
                self.status = Status::Verified;
            } else {
                let received_response = format!("{:?}", *response);

                self.status = Status::IncorrectResponse(received_response);
            }
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        match self.status {
            Status::WaitingForRequest => Ok(false),
            Status::RequestVerified => Ok(false),
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
            Status::IncorrectResponse(ref received_response) => {
                let expected_response = format!("{:?}", self.response);

                Err(
                    ErrorKind::IncorrectResponse(
                        received_response.clone(),
                        expected_response,
                    ).into(),
                )
            }
        }
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        if self.status == Status::Verified {
            return Ok(());
        }

        let expected_request = format!("{:?}", self.request);
        let expected_response = format!("{:?}", self.response);

        match self.status {
            Status::Verified => Ok(()),
            Status::WaitingForRequest => Err(
                ErrorKind::RequestAndResponseWerentVerified(
                    expected_request,
                    expected_response,
                ).into(),
            ),
            Status::RequestVerified => Err(
                ErrorKind::RequestVerifiedButNotResponse(
                    expected_request,
                    expected_response,
                ).into(),
            ),
            Status::IncorrectRequest(ref received_request) => Err(
                ErrorKind::IncorrectRequest(
                    received_request.clone(),
                    expected_request,
                ).into(),
            ),
            Status::IncorrectResponse(ref received_response) => Err(
                ErrorKind::IncorrectResponse(
                    received_response.clone(),
                    expected_response,
                ).into(),
            ),
        }
    }
}

impl<A, B> VerifierFactory for VerifyRequestResponse<A, B>
where
    A: Clone + Debug + Eq,
    B: Clone + Debug + Eq,
{
    type Verifier = Self;

    fn create(&self) -> Self::Verifier {
        VerifyRequestResponse::new(self.request.clone(), self.response.clone())
    }
}
