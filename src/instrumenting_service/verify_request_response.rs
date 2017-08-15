use super::verifier::Verifier;

#[derive(Eq, PartialEq)]
enum Status {
    WaitingForRequest,
    RequestVerified,
    Verified,
}

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
    type Error = ();

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
