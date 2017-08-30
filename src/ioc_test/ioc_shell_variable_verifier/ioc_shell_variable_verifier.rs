use std::collections::VecDeque;

use super::error_status::ErrorStatus;
use super::errors::{Error, ErrorKind};
use super::super::ioc_test_variable_action::IocTestVariableAction;
use super::super::super::instrumenting_service::verifiers::Verifier;
use super::super::super::ioc::{EpicsDataType, IocShellCommand};

pub struct IocShellVariableVerifier {
    requests: VecDeque<IocShellCommand>,
    responses: VecDeque<EpicsDataType>,
    error: Option<ErrorStatus>,
}

impl IocShellVariableVerifier {
    pub fn new(actions: Vec<IocTestVariableAction>) -> Self {
        let action_count = actions.len();
        let mut requests = VecDeque::with_capacity(action_count);
        let mut responses = VecDeque::with_capacity(action_count);

        for action in actions {
            requests.push_back(action.ioc_shell_command());
            responses.push_back(action.expected_variable_value());
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
    type Response = EpicsDataType;
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
                    self.error = Some(ErrorStatus::IncorrectVariableValue {
                        received: response.clone(),
                        expected: expected_response,
                    });
                }
            } else {
                self.error = Some(
                    ErrorStatus::UnexpectedVariableValue(response.clone()),
                );
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

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        if let Some(ref error) = self.error {
            Err(error.clone().into())
        } else if !self.requests.is_empty() || !self.responses.is_empty() {
            let unverified_results = self.responses.len();

            if let Some(first_command) = self.requests.pop_front() {
                let first_command = format!("{:?}", first_command);
                let unverified_commands = self.requests.len() + 1;
                let unverified_results_of_verified_commands =
                    unverified_results as isize - unverified_commands as isize;

                if unverified_results_of_verified_commands <= 0 {
                    Err(
                        ErrorKind::UnverifiedIocShellCommands(
                            first_command,
                            unverified_commands,
                        ).into(),
                    )
                } else {
                    Err(
                        ErrorKind::UnverifiedIocShellCommandsAndVariableValues(
                            first_command,
                            unverified_commands,
                            unverified_results_of_verified_commands as usize,
                        ).into(),
                    )
                }
            } else {
                Err(
                    ErrorKind::UnverifiedIocShellVariableValues(
                        unverified_results,
                    ).into(),
                )
            }
        } else {
            Ok(())
        }
    }
}
