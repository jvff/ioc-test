use std::io;
use std::net::AddrParseError;

use super::super::{async_server, mock_service, proxy_service, ioc, scpi};

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
    }
}
