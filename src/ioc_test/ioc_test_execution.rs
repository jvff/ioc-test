use futures::{Async, Future, Poll};
use futures::future::Flatten;

use super::errors::Error;
use super::ioc_test_protocol::IocTestProtocol;
use super::super::ioc::IocInstance;
use super::super::mock_server::ListeningMockServer;
use super::super::mock_service::MockService;

pub struct IocTestExecution<P>
where
    P: IocTestProtocol,
{
    server: Flatten<
        ListeningMockServer<P::Protocol, MockService<P::Request, P::Response>>,
    >,
    ioc: IocInstance,
}

impl<P> IocTestExecution<P>
where
    P: IocTestProtocol,
{
    pub fn new(
        ioc: IocInstance,
        server: Flatten<
            ListeningMockServer<
                P::Protocol,
                MockService<P::Request, P::Response>,
            >,
        >,
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
    P: IocTestProtocol,
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
