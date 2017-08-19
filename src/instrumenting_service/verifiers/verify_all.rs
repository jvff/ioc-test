use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

pub struct VerifyAll<V>
where
    V: Verifier,
{
    verifiers: Vec<V>,
}

impl<V> VerifyAll<V>
where
    V: Verifier,
{
    pub fn new(verifiers: Vec<V>) -> Self {
        Self { verifiers }
    }
}

impl<V> Verifier for VerifyAll<V>
where
    V: Verifier,
{
    type Request = V::Request;
    type Response = V::Response;
    type Error = V::Error;

    fn request(&mut self, request: &Self::Request) {
        for verifier in self.verifiers.iter_mut() {
            verifier.request(request);
        }
    }

    fn response(&mut self, response: &Self::Response) {
        for verifier in self.verifiers.iter_mut() {
            verifier.response(response);
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        for verifier in self.verifiers.iter() {
            match verifier.has_finished() {
                Ok(true) => {}
                hasnt_finished => return hasnt_finished,
            }
        }

        Ok(true)
    }
}

impl<V> VerifierFactory for VerifyAll<V>
where
    V: Verifier + VerifierFactory,
{
    type Verifier = VerifyAll<V::Verifier>;

    fn create(&self) -> Self::Verifier {
        let verifiers = self.verifiers
            .iter()
            .map(|factory| factory.create())
            .collect();

        VerifyAll { verifiers }
    }
}
