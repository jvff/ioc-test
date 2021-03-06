use std::{io, mem};
use std::process::ExitStatus;
use std::time::{Duration, Instant};

use futures::{Async, Future, Poll};
use tokio_core::reactor::Timeout;
use tokio_service::Service;

use super::errors::{Error, ErrorKind, Result};
use super::ioc_process::IocProcess;
use super::ioc_shell_command::IocShellCommand;
use super::ioc_shell_command_output::IocShellCommandOutput;
use super::ioc_shell_service::IocShellService;

pub struct IocInstance {
    process: IocProcess,
    shell: Option<IocShellService>,
    timeout: Option<(io::Result<Timeout>, Instant)>,
    exit_command: Option<IocShellCommandOutput>,
    error: Option<Error>,
}

impl IocInstance {
    pub fn new(process: IocProcess) -> Result<Self> {
        Ok(Self {
            process,
            shell: None,
            timeout: None,
            exit_command: None,
            error: None,
        })
    }

    pub fn shell(&mut self) -> Result<IocShellService> {
        if let Some(ref mut shell_service) = self.shell {
            Ok(shell_service.clone())
        } else {
            let shell_channel = self.process.shell()?;
            let shell_service = IocShellService::new(shell_channel);

            self.shell = Some(shell_service.clone());

            Ok(shell_service)
        }
    }

    pub fn exit(&mut self) {
        match self.shell() {
            Ok(shell_service) => {
                let command = shell_service.call(IocShellCommand::Exit);

                self.exit_command = Some(command);
            }
            Err(error) => {
                if self.error.is_none() {
                    self.error = Some(error);
                }
            }
        }
    }

    pub fn kill(&mut self) {
        if self.error.is_none() {
            self.process.kill();
        }
    }

    pub fn kill_after(&mut self, duration: Duration) {
        let new_instant = Instant::now() + duration;

        let should_set_timeout = match self.timeout {
            Some((_, ref instant)) => new_instant < *instant,
            None => true,
        };

        if should_set_timeout {
            self.set_timeout(duration, new_instant);
        }
    }

    fn set_timeout(&mut self, duration: Duration, instant: Instant) {
        let timeout_result = Timeout::new(duration, self.process.handle());

        self.timeout = Some((timeout_result, instant));
    }

    fn poll_timeout(&mut self) -> Poll<(), io::Error> {
        if let Some((timeout_spawn_result, instant)) = self.timeout.take() {
            let (poll_result, timeout_spawn_result) =
                self.poll_timeout_object(timeout_spawn_result?);

            self.timeout = Some((timeout_spawn_result, instant));

            try_ready!(poll_result);

            self.kill();
        }

        Ok(Async::Ready(()))
    }

    fn poll_timeout_object(
        &mut self,
        mut timeout: Timeout,
    ) -> (Poll<(), io::Error>, io::Result<Timeout>) {
        let poll_result = timeout.poll();

        (poll_result, Ok(timeout))
    }

    fn poll_exit_command(&mut self) -> Result<()> {
        if let Some(ref mut exit_command) = self.exit_command {
            exit_command.poll()?;
        }

        Ok(())
    }
}

impl Future for IocInstance {
    type Item = ExitStatus;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.poll_timeout()?;

        let temporary_error = ErrorKind::IocInstancePolledAfterEnd.into();
        let error_status = mem::replace(&mut self.error, Some(temporary_error));

        let (poll_result, new_error_status) = match error_status {
            None => (self.process.poll(), None),
            Some(error) => (
                Err(error),
                Some(ErrorKind::IocInstancePolledAfterEnd.into()),
            ),
        };

        let _temporary_error = mem::replace(&mut self.error, new_error_status);

        match poll_result {
            Ok(Async::NotReady) => self.poll_exit_command()?,
            _ => (),
        };

        poll_result
    }
}
