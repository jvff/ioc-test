use super::errors::Error;
use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

#[derive(Clone, Eq, PartialEq)]
enum Status {
    WaitingForRequest,
    RequestVerified,
    Verified,
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
    A: Eq,
    B: Eq,
{
    type Request = A;
    type Response = B;
    type Error = Error;

    fn request(&mut self, request: &Self::Request) {
        if self.status == Status::WaitingForRequest {
            if self.request == *request {
                self.status = Status::RequestVerified;
            }
        }
    }

    fn response(&mut self, response: &Self::Response) {
        if self.status == Status::RequestVerified {
            if self.response == *response {
                self.status = Status::Verified;
            } else {
                self.status = Status::WaitingForRequest;
            }
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        Ok(self.status == Status::Verified)
    }
}

impl<A, B> VerifierFactory for VerifyRequestResponse<A, B>
where
    A: Clone + Eq,
    B: Clone + Eq,
{
    type Verifier = Self;

    fn create(&self) -> Self::Verifier {
        VerifyRequestResponse::new(self.request.clone(), self.response.clone())
    }
}
