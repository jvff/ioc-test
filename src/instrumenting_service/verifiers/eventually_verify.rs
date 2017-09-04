use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

pub struct EventuallyVerify<V>
where
    V: VerifierFactory,
{
    factory: V,
    verified: bool,
    active_verifications: Vec<V::Verifier>,
}

impl<V> EventuallyVerify<V>
where
    V: VerifierFactory,
{
    pub fn new(factory: V) -> Self {
        Self {
            factory,
            verified: false,
            active_verifications: Vec::new(),
        }
    }

    fn new_verification(&mut self) {
        self.active_verifications.push(self.factory.create());
    }

    fn check_status(&mut self) {
        if !self.verified {
            self.verified = self.active_verifications.iter().any(
                |verification| match verification.has_finished() {
                    Ok(status) => status,
                    Err(_) => false,
                },
            );

            self.update_active_verifications();
        }
    }

    fn update_active_verifications(&mut self) {
        if self.verified {
            self.active_verifications.clear();
        } else {
            self.active_verifications
                .retain(|verification| verification.has_finished().is_ok())
        }
    }
}

impl<V> Verifier for EventuallyVerify<V>
where
    V: VerifierFactory,
{
    type Request = <V::Verifier as Verifier>::Request;
    type Response = <V::Verifier as Verifier>::Response;
    type Error = <V::Verifier as Verifier>::Error;

    fn request(&mut self, request: &Self::Request) {
        if !self.verified {
            self.new_verification();

            for verification in self.active_verifications.iter_mut() {
                verification.request(request);
            }

            self.check_status();
        }
    }

    fn response(&mut self, response: &Self::Response) {
        if !self.verified {
            self.new_verification();

            for verification in self.active_verifications.iter_mut() {
                verification.response(response);
            }

            self.check_status();
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        Ok(self.verified)
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        if self.verified {
            Ok(())
        } else {
            let mut verifications = self.active_verifications.drain(..);

            if let Some(mut oldest_active_verification) = verifications.next() {
                oldest_active_verification.force_stop()
            } else {
                self.factory.create().force_stop()
            }
        }
    }
}

impl<V> VerifierFactory for EventuallyVerify<V>
where
    V: VerifierFactory,
    V::Verifier: VerifierFactory,
{
    type Verifier = EventuallyVerify<V::Verifier>;

    fn create(&self) -> Self::Verifier {
        EventuallyVerify::new(self.factory.create())
    }
}
