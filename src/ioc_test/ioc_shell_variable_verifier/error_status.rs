use super::super::errors::{Error, ErrorKind};
use super::super::super::ioc::IocShellCommand;

#[derive(Clone)]
pub enum ErrorStatus {
    UnexpectedCommand(IocShellCommand),
    IncorrectCommand {
        received: IocShellCommand,
        expected: IocShellCommand,
    },
    UnexpectedOutput(String),
    IncorrectOutput { received: String, expected: String },
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
            ErrorStatus::UnexpectedOutput(output) => {
                ErrorKind::UnexpectedIocShellOutput(output).into()
            }
            ErrorStatus::IncorrectOutput { received, expected } => {
                ErrorKind::IncorrectIocShellOutput(
                    received.clone(),
                    expected.clone(),
                ).into()
            }
        }
    }
}
