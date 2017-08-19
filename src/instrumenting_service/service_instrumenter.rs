use std::io;
use std::ops::Deref;

use tokio_service::NewService;

use super::instrumenting_service::InstrumentingService;
use super::verifiers::{Verifier, VerifierFactory};

pub struct ServiceInstrumenter<T, V>
where
    T: NewService,
    V: Verifier<Request = T::Request, Response = T::Response>,
{
    factory: T,
    verifier: V,
}

impl<T, V> ServiceInstrumenter<T, V>
where
    T: NewService,
    V: Verifier<Request = T::Request, Response = T::Response>,
{
    pub fn new(factory: T, verifier: V) -> Self {
        Self { factory, verifier }
    }
}

impl<T, V> Deref for ServiceInstrumenter<T, V>
where
    T: NewService,
    V: Verifier<Request = T::Request, Response = T::Response>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.factory
    }
}

impl<T, V> NewService for ServiceInstrumenter<T, V>
where
    T: NewService,
    V: Verifier<Request = T::Request, Response = T::Response> + VerifierFactory,
    V::Verifier: Verifier<Request = T::Request, Response = T::Response>,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = T::Error;
    type Instance = InstrumentingService<T::Instance, V::Verifier>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let service = self.factory.new_service()?;

        Ok(Self::Instance::new(service, self.verifier.create()))
    }
}
