use std::{io, result};
use std::net::AddrParseError;

use futures::{Future, Poll, Stream};

use super::super::{mock_service, proxy_service, scpi};

error_chain! {
    links {
        MockServiceError(mock_service::Error, mock_service::ErrorKind);
        ProxyServiceError(proxy_service::Error, proxy_service::ErrorKind);
        ScpiError(scpi::Error, scpi::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
        InvalidAddressToBindTo(AddrParseError);
    }

    errors {
        AttemptToStartServerTwice {
            description(
                "server can't be started twice from same ServerStart future"
            )
        }

        AsyncServerWasShutDown {
            description("AsyncServer was shut down")
        }

        AsyncServerIsShuttingDown {
            description("AsyncServer is shutting down")
        }

        IncorrectShutDownAttempt(future: String) {
            description("incorrect attempt to shut down a completed future")
            display(
                "incorrect attempt to shut down a completed future: {}",
                future,
            )
        }

        FailedToReceiveConnection {
            description("failed to receive a connection")
        }

        FailedToBindConnection {
            description("failed to bind the connection to receive requests")
        }

        ActiveStatusHasNoPollEquivalent {
            description("active server status means processing hasn't finished")
        }
    }
}

impl<S> From<(S::Error, S)> for Error
where
    S: Stream,
    S::Error: Into<Error>,
{
    fn from(error_pair: (S::Error, S)) -> Error {
        let (error, _) = error_pair;

        error.into()
    }
}

pub trait NormalizeError<T> {
    fn normalize_error(self) -> T;
}

impl<T, E> NormalizeError<Result<T>> for result::Result<T, E>
where
    E: Into<Error>,
{
    fn normalize_error(self) -> Result<T> {
        self.map_err(|error| -> Error { error.into() })
    }
}

pub struct NormalizeErrorFuture<T>
where
    T: Future,
    T::Error: Into<Error>,
{
    future: T,
}

impl<T> Future for NormalizeErrorFuture<T>
where
    T: Future,
    T::Error: Into<Error>,
{
    type Item = T::Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.future.poll() {
            Ok(status) => Ok(status),
            Err(error) => Err(error.into()),
        }
    }
}

impl<T> NormalizeError<NormalizeErrorFuture<T>> for T
where
    T: Future,
    T::Error: Into<Error>,
{
    fn normalize_error(self) -> NormalizeErrorFuture<T> {
        NormalizeErrorFuture { future: self }
    }
}
