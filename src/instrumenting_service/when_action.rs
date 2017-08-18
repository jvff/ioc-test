use super::when_verifier::WhenVerifier;

pub trait WhenAction {
    type Request;
    type Response;

    fn when(&mut self, _request: &Self::Request) {}

    fn reply_with(&mut self, _response: &Self::Response) {}

    fn verify(
        &mut self,
        _verifier: WhenVerifier<Self::Request, Self::Response>,
    ) {
    }
}
