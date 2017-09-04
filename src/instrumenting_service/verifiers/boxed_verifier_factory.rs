use super::box_verifier_factory_wrapper::BoxVerifierFactoryWrapper;
use super::boxed_verifier::BoxedVerifier;
use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

pub struct BoxedVerifierFactory<'a, A, B, E> {
    factory:
        Box<VerifierFactory<Verifier = BoxedVerifier<'static, A, B, E>> + 'a>,
}

impl<'a, A, B, E> BoxedVerifierFactory<'a, A, B, E> {
    pub fn new<V>(factory: V) -> Self
    where
        V: VerifierFactory + 'a,
        V::Verifier: Verifier<Request = A, Response = B, Error = E> + 'static,
    {
        let wrapped_factory = BoxVerifierFactoryWrapper::from(factory);

        Self {
            factory: Box::new(wrapped_factory),
        }
    }
}

impl<'a, A, B, E> VerifierFactory for BoxedVerifierFactory<'a, A, B, E> {
    type Verifier = BoxedVerifier<'static, A, B, E>;

    fn create(&self) -> Self::Verifier {
        self.factory.create()
    }
}
