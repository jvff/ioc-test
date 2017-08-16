use super::verifier::Verifier;

pub trait WhenAction {
    type Request;
    type Response;

    fn when(&mut self, request: &Self::Request);
    fn reply_with(&mut self, response: &Self::Response);
    fn verify(
        &mut self,
        verifier: &Box<
            Verifier<
                Request = Self::Request,
                Response = Self::Response,
                Error = (),
            >,
        >,
    );
}
