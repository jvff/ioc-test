use super::ioc_test_variable_action::IocTestVariableAction;
use super::super::instrumenting_service::verifiers::{Error,
                                                     RequestResponseInterleaver,
                                                     VerifyRequestResponse,
                                                     VerifySequence, Verifier};
use super::super::ioc::{EpicsDataType, IocShellCommand};

pub struct IocShellVariableVerifier {
    verifier: RequestResponseInterleaver<
        VerifySequence<VerifyRequestResponse<IocShellCommand, EpicsDataType>>,
    >,
}

impl IocShellVariableVerifier {
    pub fn new(actions: Vec<IocTestVariableAction>) -> Self {
        let verifiers = actions.into_iter().map(|action| {
            let request = action.ioc_shell_command();
            let response = action.expected_variable_value();

            VerifyRequestResponse::new(request, response)
        });

        let sequence_verifier = VerifySequence::new(verifiers);
        let verifier = RequestResponseInterleaver::new(sequence_verifier);

        Self { verifier }
    }
}

impl Verifier for IocShellVariableVerifier {
    type Request = IocShellCommand;
    type Response = EpicsDataType;
    type Error = Error;

    fn request(&mut self, request: &Self::Request) {
        self.verifier.request(request)
    }

    fn response(&mut self, response: &Self::Response) {
        self.verifier.response(response)
    }

    fn has_finished(&self) -> Result<bool, Self::Error> {
        self.verifier.has_finished()
    }

    fn force_stop(&mut self) -> Result<(), Self::Error> {
        self.verifier.force_stop()
    }
}
