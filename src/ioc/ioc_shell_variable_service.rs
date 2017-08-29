use tokio_service::Service;

use super::epics_data_type::EpicsDataType;
use super::errors::Error;
use super::ioc_shell_command::IocShellCommand;
use super::ioc_shell_command_variable_result::IocShellCommandVariableResult;
use super::ioc_shell_service::IocShellService;

pub struct IocShellVariableService {
    shell_service: IocShellService,
}

impl From<IocShellService> for IocShellVariableService {
    fn from(shell_service: IocShellService) -> IocShellVariableService {
        IocShellVariableService { shell_service }
    }
}

impl Service for IocShellVariableService {
    type Request = IocShellCommand;
    type Response = EpicsDataType;
    type Error = Error;
    type Future = IocShellCommandVariableResult;

    fn call(&self, request: Self::Request) -> Self::Future {
        self.shell_service.call(request).into()
    }
}
