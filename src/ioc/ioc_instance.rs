use std::mem;
use std::process::ExitStatus;

use futures::{AsyncSink, Future, Poll, Sink};

use super::errors::{Error, ErrorKind, Result};
use super::ioc_process::IocProcess;
use super::ioc_shell_channel::IocShellChannel;
use super::ioc_variable_command::IocVariableCommand;

pub struct IocInstance {
    process: IocProcess,
    shell: IocShellChannel,
    error: Option<Error>,
}

impl IocInstance {
    pub fn new(mut process: IocProcess) -> Result<Self> {
        let shell = process.shell()?;

        Ok(Self {
            process,
            shell,
            error: None,
        })
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        if self.error.is_none() {
            let name = String::from(name);
            let value = String::from(value);
            let command = IocVariableCommand::Set(name, value);
            let write_error = ErrorKind::IocWriteError.into();

            self.error = match self.shell.start_send(command.into()) {
                Ok(AsyncSink::Ready) => None,
                Ok(AsyncSink::NotReady(_)) => Some(write_error),
                Err(error) => error.into(),
            }
        }
    }

    pub fn kill(&mut self) {
        if self.error.is_none() {
            self.process.kill();
        }
    }
}

impl Future for IocInstance {
    type Item = ExitStatus;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let temporary_error = ErrorKind::IocInstancePolledAfterEnd.into();
        let error_status = mem::replace(&mut self.error, Some(temporary_error));

        self.shell.poll_complete()?;

        let (poll_result, new_error_status) = match error_status {
            None => (self.process.poll(), None),
            Some(error) => (
                Err(error),
                Some(ErrorKind::IocInstancePolledAfterEnd.into()),
            ),
        };

        let _temporary_error = mem::replace(&mut self.error, new_error_status);

        poll_result
    }
}
