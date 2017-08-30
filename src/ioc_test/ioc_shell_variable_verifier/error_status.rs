use super::errors::{Error, ErrorKind};
use super::super::super::ioc::{EpicsDataType, IocShellCommand};

#[derive(Clone)]
pub enum ErrorStatus {
    UnexpectedCommand(IocShellCommand),
    IncorrectCommand {
        received: IocShellCommand,
        expected: IocShellCommand,
    },
    UnexpectedVariableValue(EpicsDataType),
    IncorrectVariableValue {
        received: EpicsDataType,
        expected: EpicsDataType,
    },
}

impl Into<Error> for ErrorStatus {
    fn into(self) -> Error {
        match self {
            ErrorStatus::UnexpectedCommand(command) => {
                ErrorKind::UnexpectedIocShellCommand(command.clone()).into()
            }
            ErrorStatus::IncorrectCommand { received, expected } => {
                ErrorKind::IncorrectIocShellCommand(
                    received.clone(),
                    expected.clone(),
                ).into()
            }
            ErrorStatus::UnexpectedVariableValue(value) => {
                ErrorKind::UnexpectedIocShellVariableValue(value).into()
            }
            ErrorStatus::IncorrectVariableValue { received, expected } => {
                ErrorKind::IncorrectIocShellVariableValue(
                    received.clone(),
                    expected.clone(),
                ).into()
            }
        }
    }
}
