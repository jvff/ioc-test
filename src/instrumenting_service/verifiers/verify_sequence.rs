use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

pub struct VerifySequence<V>
where
    V: Verifier,
{
    active_index: usize,
    verifiers: Vec<V>,
    status: Status,
}

#[derive(Clone, PartialEq)]
pub enum Status {
    Active,
    Failed,
}

impl<V> VerifySequence<V>
where
    V: Verifier,
{
    pub fn new<I>(verifiers: I) -> Self
    where
        I: IntoIterator<Item = V>,
    {
        Self {
            active_index: 0,
            verifiers: verifiers.into_iter().collect(),
            status: Status::Active,
        }
    }

    fn with_active_verifier<F>(&mut self, operation: F)
    where
        F: FnOnce(&mut V),
    {
        if self.status == Status::Active {
            if let Some(verifier) = self.verifiers.get_mut(self.active_index) {
                operation(verifier);
            }

            self.advance_finished_verifiers();
        }
    }

    fn advance_finished_verifiers(&mut self) {
        while let Some(verifier) = self.verifiers.get_mut(self.active_index) {
            match verifier.has_finished() {
                Ok(true) => self.active_index += 1,
                Ok(false) => break,
                Err(_) => {
                    self.status = Status::Failed;
                    break;
                }
            }
        }
    }
}

impl<V> Verifier for VerifySequence<V>
where
    V: Verifier,
{
    type Request = V::Request;
    type Response = V::Response;
    type Error = V::Error;

    fn request(&mut self, request: &Self::Request) {
        self.with_active_verifier(|verifier| verifier.request(request));
    }

    fn response(&mut self, response: &Self::Response) {
        self.with_active_verifier(|verifier| verifier.response(response));
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        if let Some(verifier) = self.verifiers.get(self.active_index) {
            match self.status {
                Status::Active => Ok(false),
                Status::Failed => verifier.has_finished(),
            }
        } else {
            Ok(true)
        }
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        if let Some(verifier) = self.verifiers.get_mut(self.active_index) {
            verifier.force_stop()
        } else {
            Ok(())
        }
    }
}

impl<V> VerifierFactory for VerifySequence<V>
where
    V: Verifier + VerifierFactory,
{
    type Verifier = VerifySequence<V::Verifier>;

    fn create(&self) -> VerifySequence<V::Verifier> {
        VerifySequence::new(self.verifiers.iter().map(V::create))
    }
}
