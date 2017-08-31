use super::ioc_test_parameters::IocTestParameters;
use super::ioc_test_setup::IocTestSetup;

pub trait IocTestConfigurator<P>
where
    P: IocTestParameters,
{
    fn configure(&mut self, test: &mut IocTestSetup<P>);
}

impl<F, P> IocTestConfigurator<P> for F
where
    F: Fn(&mut IocTestSetup<P>),
    P: IocTestParameters,
{
    fn configure(&mut self, test: &mut IocTestSetup<P>) {
        self(test)
    }
}
