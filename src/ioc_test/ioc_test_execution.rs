use futures::{Async, Future, Poll};
use futures::future::Flatten;

use super::errors::Error;
use super::ioc_test_parameters::IocTestParameters;
use super::super::ioc::IocInstance;
use super::super::async_server;
use super::super::async_server::ListeningServer;

pub struct IocTestExecution<P>
where
    P: IocTestParameters,
    P::ServiceError: Into<async_server::Error>,
{
    server: Flatten<ListeningServer<P::Protocol, P::Service>>,
    ioc: IocInstance,
}

impl<P> IocTestExecution<P>
where
    P: IocTestParameters,
{
    pub fn new(
        ioc: IocInstance,
        server: Flatten<ListeningServer<P::Protocol, P::Service>>,
    ) -> Self {
        Self { ioc, server }
    }

    fn poll_ioc(&mut self) -> Poll<(), Error> {
        match self.ioc.poll() {
            Ok(Async::Ready(_)) => Ok(Async::Ready(())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err(error.into()),
        }
    }

    fn kill_ioc(&mut self) -> Poll<(), Error> {
        self.ioc.kill();

        self.poll_ioc()
    }
}

impl<P> Future for IocTestExecution<P>
where
    P: IocTestParameters,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let poll_result = self.server.poll();

        match poll_result {
            Ok(Async::Ready(_)) => self.kill_ioc(),
            Ok(Async::NotReady) => self.poll_ioc(),
            Err(error) => Err(error.into()),
        }
    }
}
