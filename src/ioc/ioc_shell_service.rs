use std::sync::{Arc, Mutex};

use tokio_service::Service;

use super::errors::Error;
use super::ioc_shell_channel::IocShellChannel;
use super::ioc_shell_command::IocShellCommand;
use super::ioc_shell_command_output::IocShellCommandOutput;
use super::ioc_shell_service_scheduler::IocShellServiceScheduler;

pub struct IocShellService {
    scheduler: Arc<Mutex<IocShellServiceScheduler>>,
}

impl IocShellService {
    pub fn new(ioc_shell: IocShellChannel) -> Self {
        Self {
            scheduler: Arc::new(
                Mutex::new(IocShellServiceScheduler::new(ioc_shell)),
            ),
        }
    }
}

impl Service for IocShellService {
    type Request = IocShellCommand;
    type Response = String;
    type Error = Error;
    type Future = IocShellCommandOutput;

    fn call(&self, request: Self::Request) -> Self::Future {
        let scheduler_handle = self.scheduler.clone();

        if let Ok(mut scheduler) = self.scheduler.lock() {
            scheduler.request(request, scheduler_handle)
        } else {
            IocShellCommandOutput::with_scheduler_lock_error(scheduler_handle)
        }
    }
}
