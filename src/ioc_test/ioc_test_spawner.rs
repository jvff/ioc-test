use std::ops::Range;

use tokio_core::reactor::Handle;

use super::ioc_test_configurator::IocTestConfigurator;
use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_setup::IocTestSetup;
use super::super::test::test_spawner::TestSpawner;

pub struct IocTestSpawner<P, C>
where
    P: IocTestParameters,
    C: IocTestConfigurator<P>,
{
    handle: Handle,
    ioc_command: String,
    ports: Range<u16>,
    configurator: C,
    test_parameters: P,
}

impl<P, C> IocTestSpawner<P, C>
where
    P: IocTestParameters,
    C: IocTestConfigurator<P>,
{
    pub fn new(
        ioc_command: &str,
        ports: Range<u16>,
        handle: Handle,
        configurator: C,
        test_parameters: P,
    ) -> Self {
        Self {
            handle,
            ports,
            configurator,
            test_parameters,
            ioc_command: String::from(ioc_command),
        }
    }
}

impl<P, C> TestSpawner for IocTestSpawner<P, C>
where
    P: IocTestParameters + Clone,
    C: IocTestConfigurator<P>,
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

        self.configurator.configure(&mut test);

        test
    }
}
