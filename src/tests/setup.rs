use std::ops::Range;

use tokio_core::net::TcpStream;
use tokio_core::reactor::{Core, Handle};
use tokio_proto::pipeline::ServerProto;

use super::super::ioc_test::{Error, IocTestSetup, Result};
use super::super::scpi;
use super::super::scpi::ScpiProtocol;
use super::super::scpi::ScpiRequest;
use super::super::scpi::ScpiResponse;
use super::super::test_result::TestResult;
use super::super::test_scheduler::TestScheduler;
use super::super::test_spawner::TestSpawner;

pub trait Protocol
    : ServerProto<
    TcpStream,
    Request = ScpiRequest,
    Response = ScpiResponse,
    Error = scpi::Error,
> {
}

impl Protocol for ScpiProtocol {}

macro_rules! tests {
    ( $( $test:ident ($name:expr) $body:tt )* ) => {
        pub fn add_tests<S, P>(scheduler: &mut TestScheduler<S>)
        where
            S: TestSpawner<TestSetup = IocTestSetup<P>>,
            P: Protocol,
        {
            $(scheduler.add(|mut $test| {
                $test.name($name);
                $body
            });)*
        }
    }
}

pub struct IocTestSpawner {
    handle: Handle,
    ports: Range<u16>,
}

impl IocTestSpawner {
    pub fn new(handle: Handle) -> Self {
        let ports = 55000..56000;

        Self { handle, ports }
    }
}

impl TestSpawner for IocTestSpawner {
    type TestSetup = IocTestSetup<ScpiProtocol>;

    fn spawn(&mut self) -> Self::TestSetup {
        let port = self.ports.next().unwrap();
        let test = IocTestSetup::new(self.handle.clone(), ScpiProtocol, port);
        let mut test = test.unwrap();

        configure_initial_test_messages(&mut test);

        test
    }
}

pub fn run_tests() -> Result<Vec<TestResult<Error>>> {
    let mut reactor = Core::new()?;
    let spawner = IocTestSpawner::new(reactor.handle());
    let mut tests = TestScheduler::new(spawner);

    super::add_tests(&mut tests);

    Ok(reactor.run(tests).unwrap())
}

fn configure_initial_test_messages<P: Protocol>(test: &mut IocTestSetup<P>) {
    request_response_map! { test,
        ScpiRequest::OutputStatus(1) => ScpiResponse::Integer(0),
        ScpiRequest::SourceFrequencyGet(1) => ScpiResponse::Integer(1),
        ScpiRequest::SourcePhaseGet(1) => ScpiResponse::Integer(1),
        ScpiRequest::SourceVoltageGet(1) => ScpiResponse::Integer(1),
        ScpiRequest::SourceVoltageOffsetGet(1) => ScpiResponse::Integer(1),
        ScpiRequest::SourceFunctionQuery(1) =>
            ScpiResponse::Utf8String(String::from("SQUare")),
        ScpiRequest::SourceArbitraryFunctionFileQuery(1) =>
            ScpiResponse::Utf8String(String::from("\"DUMMY.FILE\"")),
        ScpiRequest::SourceArbitraryFunctionSampleRateGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourceNoiseFunctionBandwidthGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourcePrbsFunctionBitRateGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourcePrbsFunctionPolynomialGet(1) =>
            ScpiResponse::Utf8String(String::from("PN7")),
        ScpiRequest::SourcePrbsFunctionTransitionGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourcePulseFunctionLeadingEdgeTransitionGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourcePulseFunctionTrailingEdgeTransitionGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourcePulseFunctionPulseWidthGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourceRampFunctionSymmetryGet(1) =>
            ScpiResponse::Integer(1),
        ScpiRequest::SourceSquareFunctionDutyCycleGet(1) =>
            ScpiResponse::Integer(1),
    };
}
