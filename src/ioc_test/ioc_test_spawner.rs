use std::ops::Range;

use tokio_core::reactor::Handle;

use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_setup::IocTestSetup;
use super::mock_test_parameters::MockTestParameters;
use super::super::scpi::ScpiProtocol;
use super::super::scpi::ScpiRequest;
use super::super::scpi::ScpiResponse;
use super::super::test::test_spawner::TestSpawner;

pub struct IocTestSpawner<F>
where
    F: Fn(&mut IocTestSetup<MockTestParameters<ScpiProtocol>>)
{
    handle: Handle,
    ioc_command: String,
    ports: Range<u16>,
    setup: F,
}

impl<F> IocTestSpawner<F>
where
    F: Fn(&mut IocTestSetup<MockTestParameters<ScpiProtocol>>)
{
    pub fn new(ioc_command: &str, handle: Handle, setup: F) -> Self {
        let ports = 55000..60000;

        Self {
            handle,
            ports,
            setup,
            ioc_command: String::from(ioc_command),
        }
    }
}

impl<F> TestSpawner for IocTestSpawner<F>
where
    F: Fn(&mut IocTestSetup<MockTestParameters<ScpiProtocol>>)
{
    type TestSetup = IocTestSetup<MockTestParameters<ScpiProtocol>>;

    fn spawn(&mut self) -> Self::TestSetup {
        let handle = self.handle.clone();
        let ioc_command = self.ioc_command.as_str();
        let ip_port = self.ports.next().unwrap();
        let ca_server_port = self.ports.next().unwrap();

        let test = IocTestSetup::new(
            handle,
            ScpiProtocol,
            ioc_command,
            ip_port,
            ca_server_port,
        );
        let mut test = test.unwrap();

        (self.setup)(&mut test);

        test
    }
}
