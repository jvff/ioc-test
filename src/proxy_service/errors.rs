use std::io;

use super::super::instrumenting_service::verifiers;

error_chain! {
    foreign_links {
        IoError(io::Error);
    }

    links {
        VerifierError(verifiers::Error, verifiers::ErrorKind);
    }
}
