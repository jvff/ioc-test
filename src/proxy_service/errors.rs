use super::super::instrumenting_service::verifiers;

error_chain! {
    links {
        VerifierError(verifiers::Error, verifiers::ErrorKind);
    }
}
