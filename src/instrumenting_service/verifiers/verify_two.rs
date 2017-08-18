use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

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

impl<A, B> VerifierFactory for VerifyTwo<A, B>
where
    A: Verifier + VerifierFactory,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>
        + VerifierFactory,
    B::Verifier: Verifier<
        Request = <A::Verifier as Verifier>::Request,
        Response = <A::Verifier as Verifier>::Response,
        Error = <A::Verifier as Verifier>::Error,
    >,
{
    type Verifier = VerifyTwo<A::Verifier, B::Verifier>;

    fn create(&mut self) -> Self::Verifier {
        VerifyTwo::new(self.first.create(), self.second.create())
    }
}
