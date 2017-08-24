use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use futures::{Async, Poll, Sink, Stream};

use super::errors::{Error, ErrorKind};
use super::ioc_shell_channel::IocShellChannel;
use super::ioc_shell_command::IocShellCommand;
use super::ioc_shell_command_output::IocShellCommandOutput;

pub struct IocShellServiceScheduler {
    this: Option<Arc<Mutex<IocShellServiceScheduler>>>,
    ioc_shell: IocShellChannel,
    output_queue: VecDeque<Rc<Cell<Option<String>>>>,
}

impl IocShellServiceScheduler {
    pub fn new(ioc_shell: IocShellChannel) -> Arc<Mutex<Self>> {
        let scheduler = IocShellServiceScheduler {
            this: None,
            ioc_shell,
            output_queue: VecDeque::new(),
        };

        let scheduler_reference = Arc::new(Mutex::new(scheduler));
        let this = scheduler_reference.clone();

        if let Ok(mut scheduler) = scheduler_reference.lock() {
            scheduler.this = Some(this);
        }

        scheduler_reference
    }

    pub fn poll(&mut self) -> Poll<(), Error> {
        let mut poll_result = self.ioc_shell.poll();

        while let Ok(Async::Ready(Some(output))) = poll_result {
            if let Some(next_output_cell) = self.output_queue.pop_front() {
                next_output_cell.replace(Some(output));

                poll_result = self.ioc_shell.poll();
            } else {
                return Err(ErrorKind::UnexpectedIocShellOutput.into());
            }
        }

        match poll_result {
            Ok(Async::Ready(Some(_))) => unreachable!(),
            Ok(Async::Ready(None)) => Ok(Async::Ready(())),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(error) => Err(error),
        }
    }

    pub fn request(
        &mut self,
        request: IocShellCommand,
    ) -> IocShellCommandOutput {
        let send_result = self.ioc_shell.start_send(request);
        let scheduler = self.this.clone();
        let output_cell = Rc::new(Cell::new(None));

        self.output_queue.push_back(output_cell.clone());

        IocShellCommandOutput::new(send_result, scheduler, output_cell)
    }
}
