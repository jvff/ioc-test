use futures::{Async, Future, Poll};

use super::errors::Error;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_execution::IocTestExecution;
use super::super::ioc::IocInstance;
use super::super::ioc::IocProcess;
use super::super::ioc::IocSpawn;
use super::super::async_server::ListeningServer;

pub struct IocTestStartIoc<P>
where
    P: IocTestParameters,
{
    ioc: IocSpawn,
    listening_server: Option<ListeningServer<P::Protocol, P::Service>>,
    ioc_variables_to_set: Option<Vec<(String, String)>>,
}

impl<P> IocTestStartIoc<P>
where
    P: IocTestParameters,
{
    pub fn new(
        ioc: IocSpawn,
        listening_server: ListeningServer<P::Protocol, P::Service>,
        ioc_variables_to_set: Vec<(String, String)>,
    ) -> Self {
        Self {
            ioc,
            ioc_variables_to_set: Some(ioc_variables_to_set),
            listening_server: Some(listening_server),
        }
    }
}

impl<P> Future for IocTestStartIoc<P>
where
    P: IocTestParameters,
{
    type Item = IocTestExecution<P>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let ioc_process = try_ready!(self.ioc.poll());
        let ioc = IocInstance::new(IocProcess::new(ioc_process)?)?;

        let listening_server = self.listening_server
            .take()
            .expect("IocTestStartIoc polled after it finished");
        let server = listening_server.flatten();

        let ioc_variables_to_set = self.ioc_variables_to_set
            .take()
            .expect("IocTestStartIoc polled after it finished");

        Ok(Async::Ready(
            IocTestExecution::new(ioc, server, ioc_variables_to_set)?,
        ))
    }
}
