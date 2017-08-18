use super::verifier::Verifier;

pub struct BoxedVerifier<A, B, E> {
    verifier: Box<Verifier<Request = A, Response = B, Error = E>>,
}

impl<A, B, E> BoxedVerifier<A, B, E> {
    pub fn from<V>(verifier: V) -> Self
    where
        V: Verifier<Request = A, Response = B, Error = E> + 'static,
    {
        Self {
            verifier: Box::new(verifier),
        }
    }
}

impl<A, B, E> Verifier for BoxedVerifier<A, B, E> {
    type Request = A;
    type Response = B;
    type Error = E;

    fn request(&mut self, request: &Self::Request) {
        self.verifier.request(request);
    }

    fn response(&mut self, response: &Self::Response) {
        self.verifier.response(response);
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        self.verifier.has_finished()
    }
}
