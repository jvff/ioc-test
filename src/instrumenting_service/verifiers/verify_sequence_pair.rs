use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

#[derive(Clone)]
pub struct VerifySequencePair<A, B>
where
    A: Verifier,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>,
{
    first: A,
    second: B,
}

impl<A, B> VerifySequencePair<A, B>
where
    A: Verifier,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>,
{
    pub fn new(first: A, second: B) -> Self {
        Self { first, second }
    }
}

impl<A, B> Verifier for VerifySequencePair<A, B>
where
    A: Verifier,
    B: Verifier<Request = A::Request, Response = A::Response, Error = A::Error>,
{
    type Request = A::Request;
    type Response = A::Response;
    type Error = A::Error;

    fn request(&mut self, request: &Self::Request) {
        match self.first.has_finished() {
            Ok(false) => self.first.request(request),
            Ok(true) => self.second.request(request),
            Err(_) => {}
        }
    }

    fn response(&mut self, response: &Self::Response) {
        match self.first.has_finished() {
            Ok(false) => self.first.response(response),
            Ok(true) => self.second.response(response),
            Err(_) => {}
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        match self.first.has_finished() {
            Ok(true) => self.second.has_finished(),
            result => result,
        }
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        self.first.force_stop().and(self.second.force_stop())
    }
}

impl<A, B> VerifierFactory for VerifySequencePair<A, B>
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
    type Verifier = VerifySequencePair<A::Verifier, B::Verifier>;

    fn create(&self) -> Self::Verifier {
        VerifySequencePair::new(self.first.create(), self.second.create())
    }
}
