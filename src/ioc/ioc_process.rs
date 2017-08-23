use std::mem;
use std::process::ExitStatus;

use futures::{Future, Poll};
use tokio_process::Child;

use super::errors::{Error, ErrorKind, Result};
use super::ioc_shell_channel::IocShellChannel;

#[derive(Debug)]
pub struct IocProcess {
    process: Child,
    error: Option<Error>,
}

impl IocProcess {
    pub fn new(process: Child) -> Result<Self> {
        Ok(Self {
            process,
            error: None,
        })
    }

    pub fn shell(&mut self) -> Result<IocShellChannel> {
        IocShellChannel::from(&mut self.process)
            .ok_or(ErrorKind::IocShellAccessError.into())
    }

    pub fn kill(&mut self) {
        if let Err(error) = self.process.kill() {
            if self.error.is_none() {
                self.error = Some(error.into());
            }
        }
    }

    fn check_error(&mut self) -> Result<()> {
        let temporary_error = ErrorKind::IocProcessPolledWhileCheckingForError;
        let error_status =
            mem::replace(&mut self.error, Some(temporary_error.into()));

        let (result, new_error_status) = if let Some(error) = error_status {
            (Err(error), Some(ErrorKind::IocProcessPolledAfterEnd.into()))
        } else {
            (Ok(()), None)
        };

        let _temporary_error = mem::replace(&mut self.error, new_error_status);

        result
    }
}

impl Future for IocProcess {
    type Item = ExitStatus;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.check_error()?;

        Ok(self.process.poll()?)
    }
}
