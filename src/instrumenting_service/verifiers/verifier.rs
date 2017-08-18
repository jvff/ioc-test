use super::boxed_verifier::BoxedVerifier;
use super::converted_error::ConvertedError;

pub trait Verifier {
    type Request;
    type Response;
    type Error;

    fn request(&mut self, request: &Self::Request);
    fn response(&mut self, response: &Self::Response);
    fn has_finished(&self) -> Result<bool, Self::Error>;

    fn boxed<'a>(
        self,
    ) -> BoxedVerifier<'a, Self::Request, Self::Response, Self::Error>
    where
        Self: Sized + 'a,
    {
        BoxedVerifier::from(self)
    }

    fn convert_error<E>(self) -> ConvertedError<Self, E>
    where
        E: From<Self::Error>,
        Self: Sized,
    {
        ConvertedError::new(self)
    }
}
