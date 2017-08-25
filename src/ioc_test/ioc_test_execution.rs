use std::time::Duration;

use futures::future;
use futures::{Async, Future, Poll};
use futures::future::JoinAll;
use tokio_service::Service;

use super::errors::{Error, ErrorKind, Result};
use super::ioc_shell_variable_verifier::IocShellVariableVerifier;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_variable_action::IocTestVariableAction;
use super::super::async_server;
use super::super::async_server::{AsyncServer, FiniteService};
use super::super::ioc::{IocInstance, IocShellCommandOutput, IocShellService};
use super::super::instrumenting_service::{InstrumentedResponse,
                                          InstrumentingService};

pub struct IocTestExecution<P>
where
    P: IocTestParameters,
    P::ServiceError: Into<async_server::Error>,
{
    server: AsyncServer<P::Protocol, P::ServiceFactory>,
    ioc: IocInstance,
    service:
        InstrumentingService<IocShellService, IocShellVariableVerifier, Error>,
    commands: JoinAll<
        Vec<
            InstrumentedResponse<
                IocShellCommandOutput,
                IocShellVariableVerifier,
                Error,
            >,
        >,
    >,
}

impl<P> IocTestExecution<P>
where
    P: IocTestParameters,
{
    pub fn new(
        mut ioc: IocInstance,
        server: AsyncServer<P::Protocol, P::ServiceFactory>,
        variable_actions: Vec<IocTestVariableAction>,
    ) -> Result<Self> {
        let verifier = IocShellVariableVerifier::new(variable_actions.clone());
        let ioc_service = ioc.shell()?;
        let service = InstrumentingService::new(ioc_service, verifier);

        let command_futures = variable_actions
            .iter()
            .map(|variable_action| {
                service.call(variable_action.ioc_shell_command())
            })
            .collect();
        let commands = future::join_all(command_futures);

        Ok(Self {
            ioc,
            server,
            service,
            commands,
        })
    }

    fn poll_ioc(&mut self) -> Poll<(), Error> {
        let poll_result = self.ioc.poll();

        match poll_result {
            Ok(Async::Ready(_)) => self.ensure_ioc_service_finished(),
            Ok(Async::NotReady) => self.poll_ioc_commands(),
            Err(error) => Err(error.into()),
        }
    }

    fn poll_ioc_commands(&mut self) -> Poll<(), Error> {
        match self.commands.poll() {
            Ok(_) => Ok(Async::NotReady),
            Err(error) => Err(error.into()),
        }
    }

    fn kill_ioc(&mut self) -> Poll<(), Error> {
        self.ioc.kill_after(Duration::from_secs(5));

        self.poll_ioc()
    }

    fn poll_ioc_service(&mut self) -> Poll<(), Error> {
        match self.service.has_finished() {
            Ok(true) => Ok(Async::Ready(())),
            Ok(false) => Ok(Async::NotReady),
            Err(error) => Err(error.into()),
        }
    }

    fn ensure_ioc_service_finished(&mut self) -> Poll<(), Error> {
        match self.service.has_finished() {
            Ok(true) => Ok(self.server.shutdown()?),
            Ok(false) => Err(ErrorKind::IncompleteIocShellVerification.into()),
            Err(error) => Err(error.into()),
        }
    }
}

impl<P> Future for IocTestExecution<P>
where
    P: IocTestParameters,
{
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.poll_ioc_service()?;

        let poll_result = self.server.poll();

        match poll_result {
            Ok(Async::Ready(_)) => self.kill_ioc(),
            Ok(Async::NotReady) => self.poll_ioc(),
            Err(error) => Err(error.into()),
        }
    }
}
