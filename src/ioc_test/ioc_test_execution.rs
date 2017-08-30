use std::time::Duration;

use futures::future;
use futures::{Async, Future, Poll};
use futures::future::JoinAll;
use tokio_service::Service;

use super::errors::{Error, Result};
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
    server: Option<AsyncServer<P::Protocol, P::ServiceFactory>>,
    ioc: Option<IocInstance>,
    service: Option<
        InstrumentingService<IocShellService, IocShellVariableVerifier, Error>,
    >,
    commands: Option<
        JoinAll<
            Vec<
                InstrumentedResponse<
                    IocShellCommandOutput,
                    IocShellVariableVerifier,
                    Error,
                >,
            >,
        >,
    >,
    error: Option<Error>,
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
            ioc: Some(ioc),
            server: Some(server),
            service: Some(service),
            commands: Some(commands),
            error: None,
        })
    }

    fn poll_slot<F>(slot: &mut Option<F>, error_slot: &mut Option<Error>)
    where
        F: Future,
        F::Error: Into<Error>,
    {
        if error_slot.is_none() {
            if let Some(mut future) = slot.take() {
                let poll_result = future.poll();

                match poll_result {
                    Ok(Async::Ready(_)) => {},
                    Ok(Async::NotReady) => *slot = Some(future),
                    Err(error) => *error_slot = Some(error.into()),
                }
            }
        }
    }

    fn poll_server(&mut self) -> &mut Self {
        Self::poll_slot(&mut self.server, &mut self.error);

        self
    }

    fn poll_ioc_commands(&mut self) -> &mut Self {
        let mut command_error_slot = None;

        Self::poll_slot(&mut self.commands, &mut command_error_slot);

        if let Some(error) = command_error_slot {
            self.poll_ioc();

            if self.ioc.is_some() && self.error.is_none() {
                self.error = Some(error);
            }
        }

        self
    }

    fn poll_ioc_service(&mut self) -> &mut Self {
        if self.error.is_none() {
            if let Some(ioc_service) = self.service.take() {
                match ioc_service.has_finished() {
                    Ok(true) => {},
                    Ok(false) => self.service = Some(ioc_service),
                    Err(error) => self.error = Some(error),
                }
            }
        }

        self
    }

    fn poll_ioc(&mut self) -> &mut Self {
        Self::poll_slot(&mut self.ioc, &mut self.error);

        self
    }

    fn get_poll_result(&mut self) -> Poll<(), Error> {
        self.clean_poll_status();

        if let Some(error) = self.error.take() {
            Err(error)
        } else {
            match (&self.ioc, &self.server) {
                (&None, &None) => Ok(Async::Ready(())),
                _ => Ok(Async::NotReady),
            }
        }
    }

    fn clean_poll_status(&mut self) {
        if self.ioc.is_none() {
            self.stop_service();
            self.stop_server();
        } else if self.server.is_none() {
            self.stop_ioc();
            self.poll_ioc();
        }
    }

    fn stop_ioc(&mut self) {
        if let Some(ref mut ioc) = self.ioc {
            ioc.kill_after(Duration::from_secs(5));
            ioc.exit();
        }
    }

    fn stop_server(&mut self) {
        if self.error.is_none() {
            if let Some(mut server) = self.server.take() {
                match server.shutdown() {
                    Ok(Async::Ready(_)) => {},
                    Ok(Async::NotReady) => self.server = Some(server),
                    Err(error) => self.error = Some(error.into()),
                }
            }
        }
    }

    fn stop_service(&mut self) {
        if self.error.is_none() {
            if let Some(ref mut service) = self.service {
                match service.force_stop() {
                    Ok(_) => {},
                    Err(error) => self.error = Some(error),
                }
            }
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
        self.poll_server()
            .poll_ioc_commands()
            .poll_ioc_service()
            .poll_ioc()
            .get_poll_result()
    }
}
