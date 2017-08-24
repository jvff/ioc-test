use std::mem;
use std::process::ExitStatus;

use futures::{Future, Poll};

use super::errors::{Error, ErrorKind, Result};
use super::ioc_process::IocProcess;
use super::ioc_shell_service::IocShellService;

pub struct IocInstance {
    process: IocProcess,
    error: Option<Error>,
}

impl IocInstance {
    pub fn new(process: IocProcess) -> Result<Self> {
        Ok(Self {
            process,
            error: None,
        })
    }

    pub fn shell(&mut self) -> Result<IocShellService> {
        let shell_channel = self.process.shell()?;

        Ok(IocShellService::new(shell_channel))
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
