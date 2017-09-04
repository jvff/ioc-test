use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio_core::reactor::Handle;

use super::errors::Result;
use super::ioc_test::IocTest;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_variable_action::IocTestVariableAction;
use super::ioc_test_when_action::IocTestWhenAction;
use super::super::instrumenting_service::{When, WhenVerifier};
use super::super::instrumenting_service::verifiers::{BoxedVerifierFactory,
                                                     Verifier, VerifySequence};
use super::super::ioc::{EpicsDataType, IocSpawn};
use super::super::async_server::StartServer;
use super::super::test::test::IntoTest;

pub struct IocTestSetup<P>
where
    P: IocTestParameters,
{
    prefix: Option<String>,
    name: String,
    handle: Handle,
    request_map: HashMap<P::Request, P::Response>,
    verifiers: Vec<WhenVerifier<P::Request, P::Response>>,
    protocol: Arc<Mutex<P::Protocol>>,
    ip_address: SocketAddr,
    ca_server_port: u16,
    ioc_command: String,
    variable_actions: Vec<IocTestVariableAction>,
    test_parameters: P,
}

impl<P> IocTestSetup<P>
where
    P: IocTestParameters,
{
    pub fn new(
        handle: Handle,
        ioc_command: &str,
        ip_port: u16,
        ca_server_port: u16,
        test_parameters: P,
    ) -> Result<Self> {
        let protocol = test_parameters.create_protocol();

        Ok(Self {
            handle,
            ca_server_port,
            test_parameters,
            protocol: Arc::new(Mutex::new(protocol)),
            ip_address: SocketAddr::new("0.0.0.0".parse()?, ip_port),
            request_map: HashMap::new(),
            verifiers: Vec::new(),
            ioc_command: String::from(ioc_command),
            variable_actions: Vec::new(),
            name: String::from("Unnamed IOC test"),
            prefix: None,
        })
    }

    pub fn prefix(&mut self, prefix: &str) {
        self.prefix = Some(prefix.to_string());
    }

    pub fn name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn when<A>(
        &mut self,
        request: A,
    ) -> When<P::Request, P::Response, IocTestWhenAction<P::Request, P::Response>>
    where
        A: Into<P::Request>,
    {
        let action =
            IocTestWhenAction::new(&mut self.request_map, &mut self.verifiers);

        When::with_action(request.into(), action)
    }

    pub fn check_variable<V>(&mut self, name: &str, value: V)
    where
        V: Into<EpicsDataType>,
    {
        let name = String::from(name);
        let value = value.into();

        self.variable_actions
            .push(IocTestVariableAction::Check(name, value));
    }

    pub fn set_variable<V>(&mut self, name: &str, value: V)
    where
        V: Into<EpicsDataType>,
    {
        let name = String::from(name);
        let value = value.into();

        self.variable_actions
            .push(IocTestVariableAction::Set(name, value));
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

        let verifier_factory = BoxedVerifierFactory::new(
            VerifySequence::new(self.verifiers).eventually(),
        );
        let service_factory = self.test_parameters
            .create_service_factory(self.request_map, verifier_factory);

        let server = StartServer::new(
            self.ip_address,
            service_factory,
            self.protocol,
            self.handle,
        );

        let test_name = match self.prefix {
            Some(prefix) => format!("{} {}", prefix, self.name),
            None => self.name,
        };

        IocTest::new(test_name, ioc, server, self.variable_actions)
    }
}
