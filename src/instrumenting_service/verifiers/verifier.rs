use super::boxed_verifier::BoxedVerifier;

pub trait Verifier {
    type Request;
    type Response;
    type Error;

    fn request(&mut self, request: &Self::Request);
    fn response(&mut self, response: &Self::Response);
    fn has_finished(&self) -> Result<bool, Self::Error>;

    fn boxed(self) -> BoxedVerifier<Self::Request, Self::Response, Self::Error>
    where
        Self: Sized + 'static,
    {
        BoxedVerifier::from(self)
    }
}
