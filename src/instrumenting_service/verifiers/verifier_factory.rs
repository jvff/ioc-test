use super::verifier::Verifier;

pub trait VerifierFactory {
    type Verifier: Verifier;

    fn create(&mut self) -> Self::Verifier;
}
