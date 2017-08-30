use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_setup::IocTestSetup;

pub trait IocTestConfigurator<P>
where
    P: IocTestParameters,
{
    fn configure(&mut self, test: &mut IocTestSetup<P>);
}
