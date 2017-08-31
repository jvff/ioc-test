#![recursion_limit="128"]

extern crate bytes;
#[macro_use]
extern crate futures;
extern crate ordered_float;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_process;
extern crate tokio_service;

#[macro_use]
extern crate error_chain;

extern crate termion;

#[macro_use]
mod mock_service;

mod proxy_service;

mod instrumenting_service;

mod async_server;

mod ioc;
mod ioc_test;

mod test;

pub mod line;
pub mod scpi;

pub use self::ioc_test::{IocTestConfigurator, IocTestParameters, IocTestSetup,
                         IocTestSpawner, MockTestParameters,
                         ProxyTestParameters};
pub use self::test::parallel_test_scheduler::ParallelTestScheduler;
pub use self::test::sequential_test_scheduler::SequentialTestScheduler;
pub use self::test::test_reporter::TestReporter;
pub use self::test::test_spawner::TestSpawner;
