use super::boxed_verifier::BoxedVerifier;

pub trait WhenAction {
    type Request;
    type Response;

    fn when(&mut self, _request: &Self::Request) {}

    fn reply_with(&mut self, _response: &Self::Response) {}

    fn verify(
        &mut self,
        _verifier: BoxedVerifier<Self::Request, Self::Response, ()>,
    ) {
    }
}
