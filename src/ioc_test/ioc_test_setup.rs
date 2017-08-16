use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio_core::reactor::Handle;

use super::errors::Result;
use super::ioc_test::IocTest;
use super::ioc_test_protocol::IocTestProtocol;
use super::ioc_test_when_action::IocTestWhenAction;
use super::super::instrumenting_service::When;
use super::super::ioc::IocSpawn;
use super::super::mock_server::MockServer;
use super::super::test::test::IntoTest;

pub struct IocTestSetup<P>
where
    P: IocTestProtocol,
{
    name: String,
    handle: Handle,
    server: MockServer<P::Protocol>,
    request_map: Arc<Mutex<HashMap<P::Request, P::Response>>>,
    requests_to_verify: Arc<Mutex<HashSet<P::Request>>>,
    ip_port: u16,
    ca_server_port: u16,
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
        ca_server_port: u16,
    ) -> Result<Self> {
        let address = SocketAddr::new("0.0.0.0".parse()?, ip_port);
        let server = MockServer::new(address, protocol);

        Ok(Self {
            handle,
            server,
            ip_port,
            ca_server_port,
            request_map: Arc::new(Mutex::new(HashMap::new())),
            requests_to_verify: Arc::new(Mutex::new(HashSet::new())),
            ioc_command: String::from(ioc_command),
            ioc_variables_to_set: Vec::new(),
            name: String::from("Unnamed IOC test"),
        })
    }

    pub fn name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn when<A>(
        &mut self,
        request: A,
    ) -> When<P::Request, P::Response, IocTestWhenAction<P::Request, P::Response>>
    where
        A: Into<P::Request>,
    {
        let action = IocTestWhenAction::new(
            self.request_map.clone(),
            self.requests_to_verify.clone(),
        );

        When::with_action(request.into(), action)
    }

    pub fn verify<A>(&mut self, request: A)
    where
        A: Into<P::Request>,
    {
        let mut requests_to_verify = self.requests_to_verify.lock().expect(
            "another thread panicked while holding a lock to the mock request \
             verification set",
        );

        requests_to_verify.insert(request.into());
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        let name = String::from(name);
        let value = String::from(value);

        self.ioc_variables_to_set.push((name, value));
    }

    fn configure_server(&mut self) {
        let mut request_map = self.request_map.lock().expect(
            "another thread panicked while holding a lock to the mock request \
             map",
        );
        let mut requests_to_verify = self.requests_to_verify.lock().expect(
            "another thread panicked while holding a lock to the mock request \
             verification set",
        );

        for (request, response) in request_map.drain() {
            self.server.when(request).reply_with(response);
        }

        for request in requests_to_verify.drain() {
            self.server.verify(request);
        }
    }
}

impl<P> IntoTest for IocTestSetup<P>
where
    P: IocTestProtocol,
{
    type Test = IocTest<P>;

    fn into_test(mut self) -> Self::Test {
        self.configure_server();

        let command = self.ioc_command.clone();
        let handle = self.handle.clone();
        let ip_port = self.ip_port;
        let ca_server_port = self.ca_server_port;

        let ioc = IocSpawn::new(handle, command, ip_port, ca_server_port);
        let server = self.server.start(self.handle);

        IocTest::new(self.name, ioc, server, self.ioc_variables_to_set)
    }
}
