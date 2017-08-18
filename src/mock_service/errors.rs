use std::io;
use std::sync::PoisonError;

use super::super::scpi;

error_chain! {
    foreign_links {
        IoError(io::Error);
    }

    links {
        ScpiError(scpi::Error, scpi::ErrorKind);
    }

    errors {
        ExpectedRequestQueueAccess {
            description("failed to access expected requests queue")
        }

        NoRequests {
            description("no requests received")
        }

        MissingRequest(request: String) {
            description("expected request not received")
            display("expected request '{}' not received", request)
        }

        UnexpectedRequest(request: String) {
            description("received an unexpected request")
            display("received an unexpected request: '{}'", request)
        }
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        ErrorKind::ExpectedRequestQueueAccess.into()
    }
}
