use super::verifier::Verifier;

#[derive(Clone)]
pub struct VerifyTwo<A, B>
where
    A: Verifier,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>,
{
    first: A,
    second: B,
}

impl<A, B> VerifyTwo<A, B>
where
    A: Verifier,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>,
{
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> Verifier for VerifyTwo<A, B>
where
    A: Verifier,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>,
{
    type Request = A::Request;
    type Response = A::Response;
    type Error = A::Error;

    fn request(&mut self, request: &Self::Request) {
        self.first.request(request);
        self.second.request(request);
    }

    fn response(&mut self, response: &Self::Response) {
        self.first.response(response);
        self.second.response(response);
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        Ok(self.first.has_finished()? && self.second.has_finished()?)
    }
}
