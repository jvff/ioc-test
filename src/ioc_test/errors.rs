use std::io;
use std::net::AddrParseError;

use super::super::{async_server, mock_service, ioc, scpi};

error_chain! {
    links {
        IocError(ioc::Error, ioc::ErrorKind);
        ScpiError(scpi::Error, scpi::ErrorKind);
        ServerError(async_server::Error, async_server::ErrorKind);
        ServiceError(mock_service::Error, mock_service::ErrorKind);
    }

    foreign_links {
        Io(io::Error);
        InvalidAddress(AddrParseError);
    }

    errors {
        NoIocShellInput {
            description("spawned IOC has no shell input")
        }
    }
}
