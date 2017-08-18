use std::ops::Range;

use tokio_core::reactor::Handle;

use super::ioc_test_parameters::{IocTestParameters, MockTestParameters};
use super::ioc_test_setup::IocTestSetup;
use super::super::scpi::ScpiProtocol;
use super::super::scpi::ScpiRequest;
use super::super::scpi::ScpiResponse;
use super::super::test::test_spawner::TestSpawner;

pub struct IocTestSpawner {
    handle: Handle,
    ioc_command: String,
    ports: Range<u16>,
}

impl IocTestSpawner {
    pub fn new(ioc_command: &str, handle: Handle) -> Self {
        let ports = 55000..60000;

        Self {
            handle,
            ports,
            ioc_command: String::from(ioc_command),
        }
    }
}

impl TestSpawner for IocTestSpawner {
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

        configure_initial_test_messages(&mut test);

        test
    }
}

fn configure_initial_test_messages<P>(test: &mut IocTestSetup<P>)
where
    P: IocTestParameters,
    P::Request: From<ScpiRequest>,
    P::Response: From<ScpiResponse>,
{
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
