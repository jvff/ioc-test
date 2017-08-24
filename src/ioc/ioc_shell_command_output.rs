use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use futures::{Async, AsyncSink, Future, Poll, StartSend};

use super::errors::{Error, ErrorKind};
use super::ioc_shell_service_scheduler::IocShellServiceScheduler;

pub struct IocShellCommandOutput {
    error: Option<Error>,
    output: Rc<Cell<Option<String>>>,
    scheduler: Arc<Mutex<IocShellServiceScheduler>>,
}

impl IocShellCommandOutput {
    pub fn new<T, E>(
        send_result: StartSend<T, E>,
        scheduler: Arc<Mutex<IocShellServiceScheduler>>,
        output: Rc<Cell<Option<String>>>,
    ) -> Self
    where
        E: Into<Error>,
    {
        let error = match send_result {
            Ok(AsyncSink::Ready) => None,
            Ok(AsyncSink::NotReady(_)) => Some(
                ErrorKind::IocShellWriteError.into(),
            ),
            Err(error) => Some(error.into()),
        };

        Self {
            error,
            output,
            scheduler,
        }
    }

    pub fn with_scheduler_lock_error(
        scheduler: Arc<Mutex<IocShellServiceScheduler>>,
    ) -> Self {
        Self {
            scheduler,
            output: Rc::new(Cell::new(None)),
            error: Some(ErrorKind::IocShellServiceLockError.into()),
        }
    }
}

impl Future for IocShellCommandOutput {
    type Item = String;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        if let Some(error) = self.error.take() {
            self.error =
                Some(ErrorKind::IocShellCommandOutputPolledAfterError.into());

            Err(error)
        } else if let Some(output) = self.output.take().take() {
            Ok(Async::Ready(output))
        } else if let Ok(mut scheduler) = self.scheduler.lock() {
            let poll_result = scheduler.poll();

            if let Some(output) = self.output.take().take() {
                Ok(Async::Ready(output))
            } else {
                match poll_result {
                    Ok(Async::Ready(_)) => Err(
                        ErrorKind::IocShellReadError.into(),
                    ),
                    Ok(Async::NotReady) => Ok(Async::NotReady),
                    Err(error) => Err(error.into()),
                }
            }
        } else {
            Err(ErrorKind::IocShellServiceLockError.into())
        }
    }
}
