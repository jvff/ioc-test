use std::io;
use std::net::AddrParseError;

use super::super::{async_server, mock_service, proxy_service, ioc, scpi};
use super::super::ioc::IocShellCommand;

error_chain! {
    links {
        IocError(ioc::Error, ioc::ErrorKind);
        MockServiceError(mock_service::Error, mock_service::ErrorKind);
        ProxyServiceError(proxy_service::Error, proxy_service::ErrorKind);
        ScpiError(scpi::Error, scpi::ErrorKind);
        ServerError(async_server::Error, async_server::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
        InvalidAddress(AddrParseError);
    }

    errors {
        NoIocShellInput {
            description("spawned IOC has no shell input")
        }

        UnexpectedIocShellCommand(command: IocShellCommand) {
            description("received an unexpected IOC shell command")
            display("received an unexpected IOC shell command: \"{}\"", command)
        }

        IncorrectIocShellCommand(
            received: IocShellCommand,
            expected: IocShellCommand
        ) {
            description("received an incorrect IOC shell command")
            display(
                "received an incorrect IOC shell command: received \"{}\" but \
                 expected \"{}\"",
                received,
                expected,
            )
        }

        UnexpectedIocShellOutput(output: String) {
            description("received unexpected output from IOC shell")
            display("received unexpected output from IOC shell: \"{}\"", output)
        }

        IncorrectIocShellOutput(received: String, expected: String) {
            description("received an incorrect IOC shell command")
            display(
                "received an incorrect IOC shell command: received \"{}\" but \
                 expected \"{}\"",
                received,
                expected,
            )
        }
    }
}
