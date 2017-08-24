use futures::future;
use futures::{Async, Future, Poll};
use futures::future::{Flatten, JoinAll};
use tokio_service::Service;

use super::errors::{Error, Result};
use super::ioc_test_parameters::IocTestParameters;
use super::super::ioc::{IocInstance, IocShellCommand, IocShellCommandOutput};
use super::super::async_server;
use super::super::async_server::ListeningServer;

pub struct IocTestExecution<P>
where
    P: IocTestParameters,
    P::ServiceError: Into<async_server::Error>,
{
    server: Flatten<ListeningServer<P::Protocol, P::Service>>,
    ioc: IocInstance,
    commands: JoinAll<Vec<IocShellCommandOutput>>,
}

impl<P> IocTestExecution<P>
where
    P: IocTestParameters,
{
    pub fn new(
        mut ioc: IocInstance,
        server: Flatten<ListeningServer<P::Protocol, P::Service>>,
        ioc_variables_to_set: Vec<(String, String)>,
    ) -> Result<Self> {
        let ioc_service = ioc.shell()?;
        let command_futures =
            ioc_variables_to_set.into_iter().map(|(name, value)| {
                ioc_service.call(IocShellCommand::DbPutField(name, value))
            }).collect();
        let commands = future::join_all(command_futures);

        Ok(Self {
            ioc,
            server,
            commands,
        })
    }

    fn poll_ioc(&mut self) -> Poll<(), Error> {
        let poll_result = self.ioc.poll();

        match poll_result {
            Ok(Async::Ready(_)) => Ok(Async::Ready(())),
            Ok(Async::NotReady) => self.poll_ioc_commands(),
            Err(error) => Err(error.into()),
        }
    }

    fn poll_ioc_commands(&mut self) -> Poll<(), Error> {
        match self.commands.poll() {
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
