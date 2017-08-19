use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio_core::reactor::Handle;

use super::errors::Result;
use super::ioc_test::IocTest;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_when_action::IocTestWhenAction;
use super::super::instrumenting_service::{When, WhenVerifier};
use super::super::ioc::IocSpawn;
use super::super::async_server::StartServer;
use super::super::test::test::IntoTest;

pub struct IocTestSetup<P>
where
    P: IocTestParameters,
{
    name: String,
    handle: Handle,
    request_map: HashMap<P::Request, P::Response>,
    requests_to_verify: HashSet<P::Request>,
    verifiers: Vec<WhenVerifier<P::Request, P::Response>>,
    protocol: Arc<Mutex<P::Protocol>>,
    ip_address: SocketAddr,
    ca_server_port: u16,
    ioc_command: String,
    ioc_variables_to_set: Vec<(String, String)>,
}

impl<P> IocTestSetup<P>
where
    P: IocTestParameters,
{
    pub fn new(
        handle: Handle,
        protocol: P::Protocol,
        ioc_command: &str,
        ip_port: u16,
        ca_server_port: u16,
    ) -> Result<Self> {
        Ok(Self {
            handle,
            ca_server_port,
            protocol: Arc::new(Mutex::new(protocol)),
            ip_address: SocketAddr::new("0.0.0.0".parse()?, ip_port),
            request_map: HashMap::new(),
            requests_to_verify: HashSet::new(),
            verifiers: Vec::new(),
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
            &mut self.request_map,
            &mut self.requests_to_verify,
            &mut self.verifiers,
        );

        When::with_action(request.into(), action)
    }

    pub fn verify<A>(&mut self, request: A)
    where
        A: Into<P::Request>,
    {
        self.requests_to_verify.insert(request.into());
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        let name = String::from(name);
        let value = String::from(value);

        self.ioc_variables_to_set.push((name, value));
    }
}

impl<P> IntoTest for IocTestSetup<P>
where
    P: IocTestParameters,
{
    type Test = IocTest<P>;

    fn into_test(self) -> Self::Test {
        let command = self.ioc_command.clone();
        let handle = self.handle.clone();
        let ip_port = self.ip_address.port();
        let ca_server_port = self.ca_server_port;

        let ioc = IocSpawn::new(handle, command, ip_port, ca_server_port);

        let service_factory = P::create_service_factory(
            self.request_map,
            self.requests_to_verify,
        );

        let server = StartServer::new(
            self.ip_address,
            service_factory,
            self.protocol,
            self.handle,
        );

        IocTest::new(self.name, ioc, server, self.ioc_variables_to_set)
    }
}
