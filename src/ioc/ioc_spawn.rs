use std::io;
use std::process::{Command, Stdio};

use futures::{Async, Future, Poll};
use tokio_core::reactor::Handle;
use tokio_process::{Child, CommandExt};

pub struct IocSpawn {
    handle: Handle,
    command: String,
    ip_port: u16,
    ca_server_port: u16,
}

impl IocSpawn {
    pub fn new(
        handle: Handle,
        command: String,
        ip_port: u16,
        ca_server_port: u16,
    ) -> Self {
        Self {
            handle,
            command,
            ip_port,
            ca_server_port,
        }
    }
}

impl Future for IocSpawn {
    type Item = Child;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let process = Command::new(self.command.as_str())
            .env("IPADDR", "127.0.0.1")
            .env("IPPORT", self.ip_port.to_string())
            .env("EPICS_CA_SERVER_PORT", self.ca_server_port.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .spawn_async(&self.handle)?;

        Ok(Async::Ready(process))
    }
}
