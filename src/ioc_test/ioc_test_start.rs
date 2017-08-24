use futures::{Async, Future, Poll};

use super::errors::Error;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_start_ioc::IocTestStartIoc;
use super::ioc_test_variable_action::IocTestVariableAction;
use super::super::ioc::IocSpawn;
use super::super::async_server::StartServer;

pub struct IocTestStart<P>
where
    P: IocTestParameters,
{
    ioc: Option<IocSpawn>,
    server: StartServer<P::Protocol, P::ServiceFactory>,
    variable_actions: Option<Vec<IocTestVariableAction>>,
}

impl<P> IocTestStart<P>
where
    P: IocTestParameters,
{
    pub fn new(
        ioc: IocSpawn,
        server: StartServer<P::Protocol, P::ServiceFactory>,
        variable_actions: Vec<IocTestVariableAction>,
    ) -> Self {
        Self {
            server,
            ioc: Some(ioc),
            variable_actions: Some(variable_actions),
        }
    }

    fn take_parameters_to_forward(
        &mut self,
    ) -> (IocSpawn, Vec<IocTestVariableAction>) {
        let error_message = "IocTestStart polled after it finished";

        let ioc = self.ioc.take().expect(error_message);
        let variable_actions =
            self.variable_actions.take().expect(error_message);

        (ioc, variable_actions)
    }
}

impl<P> Future for IocTestStart<P>
where
    P: IocTestParameters,
{
    type Item = IocTestStartIoc<P>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let listening_server = try_ready!(self.server.poll());

        let (ioc, variable_actions) = self.take_parameters_to_forward();

        Ok(Async::Ready(IocTestStartIoc::new(
            ioc,
            listening_server,
            variable_actions,
        )))
    }
}
