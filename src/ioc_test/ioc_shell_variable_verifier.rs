use std::collections::VecDeque;

use super::errors::{Error, ErrorKind};
use super::ioc_test_variable_action::IocTestVariableAction;
use super::super::instrumenting_service::verifiers::Verifier;
use super::super::ioc::IocShellCommand;

#[derive(Clone)]
enum ErrorStatus {
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

pub struct IocShellVariableVerifier {
    requests: VecDeque<IocShellCommand>,
    responses: VecDeque<String>,
    error: Option<ErrorStatus>,
}

impl IocShellVariableVerifier {
    pub fn new(actions: Vec<IocTestVariableAction>) -> Self {
        let action_count = actions.len();
        let mut requests = VecDeque::with_capacity(action_count);
        let mut responses = VecDeque::with_capacity(action_count);

        for action in actions {
            requests.push_back(action.ioc_shell_command());
            responses.push_back(action.expected_output());
        }

        Self {
            requests,
            responses,
            error: None,
        }
    }
}

impl Verifier for IocShellVariableVerifier {
    type Request = IocShellCommand;
    type Response = String;
    type Error = Error;

    fn request(&mut self, request: &Self::Request) {
        if self.error.is_none() {
            if let Some(expected_request) = self.requests.pop_front() {
                if *request != expected_request {
                    self.error = Some(ErrorStatus::IncorrectCommand {
                        received: request.clone(),
                        expected: expected_request,
                    });
                }
            } else {
                self.error =
                    Some(ErrorStatus::UnexpectedCommand(request.clone()));
            }
        }
    }

    fn response(&mut self, response: &Self::Response) {
        if self.error.is_none() {
            if let Some(expected_response) = self.responses.pop_front() {
                if *response != expected_response {
                    self.error = Some(ErrorStatus::IncorrectOutput {
                        received: response.clone(),
                        expected: expected_response,
                    });
                }
            } else {
                self.error =
                    Some(ErrorStatus::UnexpectedOutput(response.clone()));
            }
        }
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        if let Some(ref error) = self.error {
            Err(error.clone().into())
        } else {
            Ok(self.requests.is_empty() && self.responses.is_empty())
        }
    }
}
