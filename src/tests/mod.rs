#[macro_use]
mod setup;

mod output;

use self::setup::Protocol;
use super::ioc_test::IocTestSetup;
use super::test_scheduler::TestScheduler;
use super::test_spawner::TestSpawner;

pub use self::setup::run_tests;

pub fn add_tests<S, P>(scheduler: &mut TestScheduler<S>)
where
    S: TestSpawner<TestSetup = IocTestSetup<P>>,
    P: Protocol,
{
    output::add_tests(scheduler);
}
