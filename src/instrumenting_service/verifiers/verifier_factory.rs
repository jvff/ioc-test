use super::verifier::Verifier;

pub trait VerifierFactory {
    type Verifier: Verifier;

    fn create(&self) -> Self::Verifier;
}
