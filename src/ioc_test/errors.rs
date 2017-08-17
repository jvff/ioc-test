use std::io;
use std::net::AddrParseError;

use super::super::async_server;
use super::super::ioc;

error_chain! {
    links {
        IocError(ioc::Error, ioc::ErrorKind);
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
