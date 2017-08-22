use std::io;

use super::super::instrumenting_service::verifiers;
use super::super::scpi;

error_chain! {
    foreign_links {
        IoError(io::Error);
    }

    links {
        ScpiError(scpi::Error, scpi::ErrorKind);
        VerifierError(verifiers::Error, verifiers::ErrorKind);
    }
}
