use std::io;
use std::net::AddrParseError;

use super::ioc_shell_variable_verifier;
use super::super::{async_server, mock_service, proxy_service, ioc, scpi};
use super::super::instrumenting_service::verifiers;

error_chain! {
    links {
        IocError(ioc::Error, ioc::ErrorKind);
        IocShellVariableError(
            ioc_shell_variable_verifier::Error,
            ioc_shell_variable_verifier::ErrorKind
        );
        MockServiceError(mock_service::Error, mock_service::ErrorKind);
        ProxyServiceError(proxy_service::Error, proxy_service::ErrorKind);
        ScpiError(scpi::Error, scpi::ErrorKind);
        ServerError(async_server::Error, async_server::ErrorKind);
        VerifierError(verifiers::Error, verifiers::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
        InvalidAddress(AddrParseError);
    }

    errors {
        NoIocShellInput {
            description("spawned IOC has no shell input")
        }

        IncompleteIocShellVerification {
            description(
                "IOC was closed before IOC shell verification completed"
            )
        }
    }
}
