use std::net::SocketAddr;

use tokio_core::reactor::Handle;

use super::errors::Result;
use super::ioc_test::IocTest;
use super::ioc_test_protocol::IocTestProtocol;
use super::super::ioc::IocSpawn;
use super::super::mock_server::MockServer;
use super::super::mock_service::When;
use super::super::test::test::IntoTest;

pub struct IocTestSetup<P>
where
    P: IocTestProtocol,
{
    name: String,
    handle: Handle,
    server: MockServer<P::Protocol>,
    ip_port: u16,
    ioc_command: String,
    ioc_variables_to_set: Vec<(String, String)>,
}

impl<P> IocTestSetup<P>
where
    P: IocTestProtocol,
{
    pub fn new(
        handle: Handle,
        protocol: P::Protocol,
        ioc_command: &str,
        ip_port: u16,
    ) -> Result<Self> {
        let address = SocketAddr::new("0.0.0.0".parse()?, ip_port);
        let server = MockServer::new(address, protocol);

        Ok(Self {
            handle,
            server,
            ip_port,
            ioc_command: String::from(ioc_command),
            ioc_variables_to_set: Vec::new(),
            name: String::from("Unnamed IOC test"),
        })
    }

    pub fn name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn when<A>(&mut self, request: A) -> When<P::Request, P::Response>
    where
        A: Into<P::Request>,
    {
        self.server.when(request)
    }

    pub fn verify<A>(&mut self, request: A)
    where
        A: Into<P::Request>,
    {
        self.server.verify(request);
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        let name = String::from(name);
        let value = String::from(value);

        self.ioc_variables_to_set.push((name, value));
    }
}

impl<P> IntoTest for IocTestSetup<P>
where
    P: IocTestProtocol,
{
    type Test = IocTest<P>;

    fn into_test(self) -> Self::Test {
        let command = self.ioc_command.clone();
        let handle = self.handle.clone();
        let ioc = IocSpawn::new(handle, command, self.ip_port);
        let server = self.server.start(self.handle);

        IocTest::new(self.name, ioc, server, self.ioc_variables_to_set)
    }
}
