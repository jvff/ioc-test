use super::boxed_verifier::BoxedVerifier;
use super::verifier::Verifier;
use super::verifier_factory::VerifierFactory;

pub struct BoxVerifierFactoryWrapper<V>
where
    V: VerifierFactory,
{
    factory: V,
}

impl<V> From<V> for BoxVerifierFactoryWrapper<V>
where
    V: VerifierFactory,
{
    fn from(factory: V) -> Self {
        Self { factory }
    }
}

impl<V> VerifierFactory for BoxVerifierFactoryWrapper<V>
where
    V: VerifierFactory,
    V::Verifier: 'static,
{
    type Verifier = BoxedVerifier<
        'static,
        <V::Verifier as Verifier>::Request,
        <V::Verifier as Verifier>::Response,
        <V::Verifier as Verifier>::Error,
    >;

    fn create(&self) -> Self::Verifier {
        self.factory.create().boxed()
    }
}
