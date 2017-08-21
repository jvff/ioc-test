use std::ops::Range;

use tokio_core::reactor::Handle;

use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_setup::IocTestSetup;
use super::super::test::test_spawner::TestSpawner;

pub struct IocTestSpawner<P, F>
where
    P: IocTestParameters,
    F: Fn(&mut IocTestSetup<P>),
{
    handle: Handle,
    ioc_command: String,
    ports: Range<u16>,
    setup: F,
    test_parameters: P,
}

impl<P, F> IocTestSpawner<P, F>
where
    P: IocTestParameters,
    F: Fn(&mut IocTestSetup<P>),
{
    pub fn new(
        ioc_command: &str,
        handle: Handle,
        setup: F,
        test_parameters: P,
    ) -> Self {
        let ports = 55000..60000;

        Self {
            handle,
            ports,
            setup,
            test_parameters,
            ioc_command: String::from(ioc_command),
        }
    }
}

impl<P, F> TestSpawner for IocTestSpawner<P, F>
where
    P: IocTestParameters + Clone,
    F: Fn(&mut IocTestSetup<P>),
{
    type TestSetup = IocTestSetup<P>;

    fn spawn(&mut self) -> Self::TestSetup {
        let handle = self.handle.clone();
        let ioc_command = self.ioc_command.as_str();
        let ip_port = self.ports.next().unwrap();
        let ca_server_port = self.ports.next().unwrap();
        let test_parameters = self.test_parameters.clone();

        let test = IocTestSetup::new(
            handle,
            ioc_command,
            ip_port,
            ca_server_port,
            test_parameters,
        );
        let mut test = test.unwrap();

        (self.setup)(&mut test);

        test
    }
}
