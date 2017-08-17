use futures::{Async, Future, Poll};

use super::errors::Error;
use super::ioc_test_protocol::IocTestProtocol;
use super::ioc_test_start_ioc::IocTestStartIoc;
use super::super::ioc::IocSpawn;
use super::super::async_server::MockServerStart;
use super::super::mock_service::MockServiceFactory;

pub struct IocTestStart<P>
where
    P: IocTestProtocol,
{
    ioc: Option<IocSpawn>,
    server: MockServerStart<
        P::Protocol,
        MockServiceFactory<P::Request, P::Response>,
    >,
    ioc_variables_to_set: Option<Vec<(String, String)>>,
}

impl<P> IocTestStart<P>
where
    P: IocTestProtocol,
{
    pub fn new(
        ioc: IocSpawn,
        server: MockServerStart<
            P::Protocol,
            MockServiceFactory<P::Request, P::Response>,
        >,
        ioc_variables_to_set: Vec<(String, String)>,
    ) -> Self {
        Self {
            server,
            ioc: Some(ioc),
            ioc_variables_to_set: Some(ioc_variables_to_set),
        }
    }

    fn take_parameters_to_forward(
        &mut self,
    ) -> (IocSpawn, Vec<(String, String)>) {
        let error_message = "IocTestStart polled after it finished";

        let ioc = self.ioc.take().expect(error_message);
        let ioc_variables_to_set =
            self.ioc_variables_to_set.take().expect(error_message);

        (ioc, ioc_variables_to_set)
    }
}

impl<P> Future for IocTestStart<P>
where
    P: IocTestProtocol,
{
    type Item = IocTestStartIoc<P>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let listening_server = try_ready!(self.server.poll());

        let (ioc, ioc_variables_to_set) = self.take_parameters_to_forward();

        Ok(Async::Ready(IocTestStartIoc::new(
            ioc,
            listening_server,
            ioc_variables_to_set,
        )))
    }
}
