pub trait Verifier {
    type Request;
    type Response;
    type Error;

    fn request(&mut self, request: &Self::Request);
    fn response(&mut self, response: &Self::Response);
    fn has_finished(&self) -> Result<bool, Self::Error>;
}
