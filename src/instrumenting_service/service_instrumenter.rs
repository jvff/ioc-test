use std::marker::PhantomData;
use std::io;
use std::ops::Deref;

use tokio_service::{NewService, Service};

use super::instrumenting_service::InstrumentingService;
use super::verifiers::{Verifier, VerifierFactory};

pub struct ServiceInstrumenter<T, V, E>
where
    T: NewService,
    V: VerifierFactory,
    V::Verifier: Verifier<Request = T::Request, Response = T::Response>,
    E: From<T::Error>
        + From<<V::Verifier as Verifier>::Error>
        + From<<T::Instance as Service>::Error>,
{
    factory: T,
    verifier: V,
    _error_type: PhantomData<E>,
}

impl<T, V, E> ServiceInstrumenter<T, V, E>
where
    T: NewService,
    V: VerifierFactory,
    V::Verifier: Verifier<Request = T::Request, Response = T::Response>,
    E: From<T::Error>
        + From<<V::Verifier as Verifier>::Error>
        + From<<T::Instance as Service>::Error>,
{
    pub fn new(factory: T, verifier: V) -> Self {
        Self {
            factory,
            verifier,
            _error_type: PhantomData,
        }
    }
}

impl<T, V, E> Deref for ServiceInstrumenter<T, V, E>
where
    T: NewService,
    V: VerifierFactory,
    V::Verifier: Verifier<Request = T::Request, Response = T::Response>,
    E: From<T::Error>
        + From<<V::Verifier as Verifier>::Error>
        + From<<T::Instance as Service>::Error>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.factory
    }
}

impl<T, V, E> NewService for ServiceInstrumenter<T, V, E>
where
    T: NewService,
    V: VerifierFactory,
    V::Verifier: Verifier<Request = T::Request, Response = T::Response>,
    E: From<T::Error>
        + From<<V::Verifier as Verifier>::Error>
        + From<<T::Instance as Service>::Error>,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = E;
    type Instance = InstrumentingService<T::Instance, V::Verifier, E>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let service = self.factory.new_service()?;

        Ok(Self::Instance::new(service, self.verifier.create()))
    }
}
