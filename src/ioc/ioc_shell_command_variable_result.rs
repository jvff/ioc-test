use futures::{Async, Future, Poll};

use super::epics_data_type::EpicsDataType;
use super::errors::Error;
use super::ioc_shell_command_output::IocShellCommandOutput;

pub struct IocShellCommandVariableResult {
    output: IocShellCommandOutput,
}

impl From<IocShellCommandOutput> for IocShellCommandVariableResult {
    fn from(output: IocShellCommandOutput) -> Self {
        Self { output }
    }
}

impl Future for IocShellCommandVariableResult {
    type Item = EpicsDataType;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let string = try_ready!(self.output.poll());

        Ok(Async::Ready(EpicsDataType::from(string)?))
    }
}
