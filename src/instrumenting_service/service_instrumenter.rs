use std::collections::HashSet;
use std::hash::Hash;
use std::io;
use std::ops::Deref;

use tokio_service::NewService;

use super::instrumenting_service::InstrumentingService;

pub struct ServiceInstrumenter<T>
where
    T: NewService,
{
    factory: T,
    requests_to_verify: HashSet<T::Request>,
}

impl<T> ServiceInstrumenter<T>
where
    T: NewService,
    <T as NewService>::Request: Eq + Hash,
{
    pub fn new(factory: T) -> Self {
        Self {
            factory,
            requests_to_verify: HashSet::new(),
        }
    }

    pub fn verify<C>(&mut self, request: C) -> &mut Self
    where
        C: Into<T::Request>,
    {
        self.requests_to_verify.insert(request.into());

        self
    }
}

impl<T> Deref for ServiceInstrumenter<T>
where
    T: NewService,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.factory
    }
}

impl<T> NewService for ServiceInstrumenter<T>
where
    T: NewService,
    <T as NewService>::Request: Clone + Eq + Hash,
{
    type Request = T::Request;
    type Response = T::Response;
    type Error = T::Error;
    type Instance = InstrumentingService<T::Instance>;

    fn new_service(&self) -> io::Result<Self::Instance> {
        let service = self.factory.new_service()?;

        Ok(Self::Instance::new(service, self.requests_to_verify.clone()))
    }
}
