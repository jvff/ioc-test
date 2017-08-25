use std::collections::VecDeque;

use super::error_status::ErrorStatus;
use super::errors::Error;
use super::super::ioc_test_variable_action::IocTestVariableAction;
use super::super::super::instrumenting_service::verifiers::Verifier;
use super::super::super::ioc::IocShellCommand;

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
        let response = response.trim();

        if self.error.is_none() {
            if let Some(expected_response) = self.responses.pop_front() {
                if *response != expected_response {
                    self.error = Some(ErrorStatus::IncorrectOutput {
                        received: String::from(response),
                        expected: expected_response,
                    });
                }
            } else {
                self.error =
                    Some(ErrorStatus::UnexpectedOutput(String::from(response)));
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
