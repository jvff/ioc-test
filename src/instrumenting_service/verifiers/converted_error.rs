use std::marker::PhantomData;

use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

#[derive(Clone)]
pub struct ConvertedError<V, E>
where
    V: Verifier,
    E: From<V::Error>,
{
    verifier: V,
    _new_error_type: PhantomData<E>,
}

impl<V, E> ConvertedError<V, E>
where
    V: Verifier,
    E: From<V::Error>,
{
    pub fn new(verifier: V) -> Self {
        Self {
            verifier,
            _new_error_type: PhantomData,
        }
    }
}

impl<V, E> Verifier for ConvertedError<V, E>
where
    V: Verifier,
    E: From<V::Error>,
{
    type Request = V::Request;
    type Response = V::Response;
    type Error = E;

    fn request(&mut self, request: &Self::Request) {
        self.verifier.request(request);
    }

    fn response(&mut self, response: &Self::Response) {
        self.verifier.response(response);
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        self.verifier.has_finished().map_err(|error| error.into())
    }
}

impl<V, E> VerifierFactory for ConvertedError<V, E>
where
    V: Verifier + VerifierFactory,
    E: From<V::Error> + From<<V::Verifier as Verifier>::Error>,
{
    type Verifier = ConvertedError<V::Verifier, E>;

    fn create(&self) -> Self::Verifier {
        ConvertedError::new(self.verifier.create())
    }
}
