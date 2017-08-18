extern crate bytes;
#[macro_use]
extern crate futures;
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

pub use self::ioc_test::{IocTestParameters, IocTestProtocol, IocTestSetup,
                         IocTestSpawner, MockTestParameters};
pub use self::test::test_reporter::TestReporter;
pub use self::test::test_scheduler::TestScheduler;
pub use self::test::test_spawner::TestSpawner;
