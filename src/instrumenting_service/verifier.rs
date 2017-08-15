pub trait Verifier {
    type Request;
    type Response;

    fn request(&mut self, request: &Self::Request);
    fn response(&mut self, response: &Self::Response);
    fn has_finished(&self) -> bool;
}
