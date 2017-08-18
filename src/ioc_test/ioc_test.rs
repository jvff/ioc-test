use futures::{Future, Poll};
use futures::future::Flatten;

use super::errors::Error;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_start::IocTestStart;
use super::super::async_server::StartServer;
use super::super::ioc::IocSpawn;
use super::super::test::test::Test;

pub struct IocTest<P>
where
    P: IocTestParameters,
{
    name: String,
    future: Flatten<Flatten<IocTestStart<P>>>,
}

impl<P> IocTest<P>
where
    P: IocTestParameters,
{
    pub fn new(
        name: String,
        ioc: IocSpawn,
        server: StartServer<P::Protocol, P::ServiceFactory>,
        ioc_variables_to_set: Vec<(String, String)>,
    ) -> Self {
        let test_start = IocTestStart::new(ioc, server, ioc_variables_to_set);

        Self {
            name,
            future: test_start.flatten().flatten(),
        }
    }
}

impl<P> Test for IocTest<P>
where
    P: IocTestParameters,
{
    type Error = Error;

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn poll_test(&mut self) -> Poll<(), Self::Error> {
        self.future.poll()
    }
}
