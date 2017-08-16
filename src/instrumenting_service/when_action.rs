use super::boxed_verifier::BoxedVerifier;

pub trait WhenAction {
    type Request;
    type Response;

    fn when(&mut self, request: &Self::Request);
    fn reply_with(&mut self, response: &Self::Response);
    fn verify(
        &mut self,
        verifier: &BoxedVerifier<Self::Request, Self::Response, ()>,
    );
}
