mod frequency;
mod function;
mod phase;
mod voltage;

use std::fmt;
use std::fmt::{Display, Formatter};

use super::super::request::ScpiRequest;
use super::super::str_extensions::StrExtensions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ScpiSourceSubsystem {
    SourceFrequencyGet(usize),
    SourcePhaseGet(usize),
    SourceVoltageGet(usize),
    SourceVoltageOffsetGet(usize),
    SourceFunctionQuery(usize),
    SourceArbitraryFunctionFileQuery(usize),
    SourceArbitraryFunctionSampleRateGet(usize),
    SourceNoiseFunctionBandwidthGet(usize),
    SourcePrbsFunctionBitRateGet(usize),
    SourcePrbsFunctionPolynomialGet(usize),
    SourcePrbsFunctionTransitionGet(usize),
    SourcePulseFunctionLeadingEdgeTransitionGet(usize),
    SourcePulseFunctionTrailingEdgeTransitionGet(usize),
    SourcePulseFunctionPulseWidthGet(usize),
    SourceRampFunctionSymmetryGet(usize),
    SourceSquareFunctionDutyCycleGet(usize),
}

impl Display for ScpiSourceSubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            ScpiSourceSubsystem::SourceFrequencyGet(source) => {
                write!(formatter, "SOUR{}:FREQ?", source)
            }
            ScpiSourceSubsystem::SourcePhaseGet(source) => {
                write!(formatter, "SOUR{}:PHAS?", source)
            }
            ScpiSourceSubsystem::SourceVoltageGet(source) => {
                write!(formatter, "SOUR{}:VOLT?", source)
            }
            ScpiSourceSubsystem::SourceVoltageOffsetGet(source) => {
                write!(formatter, "SOUR{}:VOLT:OFFSet?", source)
            }
            ScpiSourceSubsystem::SourceFunctionQuery(source) => {
                write!(formatter, "SOUR{}:FUNC?", source)
            }
            ScpiSourceSubsystem::SourceArbitraryFunctionFileQuery(source) => {
                write!(formatter, "SOUR{}:FUNC:ARB?", source)
            }
            ScpiSourceSubsystem::SourceArbitraryFunctionSampleRateGet(source) => {
                write!(formatter, "SOUR{}:FUNC:ARB:SRAT?", source)
            }
            ScpiSourceSubsystem::SourceNoiseFunctionBandwidthGet(source) => {
                write!(formatter, "SOUR{}:FUNC:NOIS:BAND?", source)
            }
            ScpiSourceSubsystem::SourcePrbsFunctionBitRateGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PRBS:BRAT?", source)
            }
            ScpiSourceSubsystem::SourcePrbsFunctionPolynomialGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PRBS:DATA?", source)
            }
            ScpiSourceSubsystem::SourcePrbsFunctionTransitionGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PRBS:TRAN?", source)
            }
            ScpiSourceSubsystem::SourcePulseFunctionLeadingEdgeTransitionGet(
                source,
            ) => write!(formatter, "SOUR{}:FUNC:PULS:TRAN:LEAD?", source),
            ScpiSourceSubsystem::SourcePulseFunctionTrailingEdgeTransitionGet(
                source,
            ) => write!(formatter, "SOUR{}:FUNC:PULS:TRAN:TRA?", source),
            ScpiSourceSubsystem::SourcePulseFunctionPulseWidthGet(source) => {
                write!(formatter, "SOUR{}:FUNC:PULS:WIDT?", source)
            }
            ScpiSourceSubsystem::SourceRampFunctionSymmetryGet(source) => {
                write!(formatter, "SOUR{}:FUNC:RAMP:SYMM?", source)
            }
            ScpiSourceSubsystem::SourceSquareFunctionDutyCycleGet(source) => {
                write!(formatter, "SOUR{}:FUNC:SQU:DCYC?", source)
            }
        }
    }
}

impl ScpiRequest for ScpiSourceSubsystem {
    fn decode(message: &str) -> Option<Self> {
        if message.view_first_chars(4) == "SOUR" {
            return decode_source_message(message);
        }

        None
    }
}

pub fn decode_source_message(string: &str) -> Option<ScpiSourceSubsystem> {
    let request_data = string.skip_expected_chars("SOURce");

    if let Some((source, command)) = request_data.parse_integer() {
        if command.starts_with(":") {
            let command = command.skip_chars(1);

            match command.view_first_chars(4) {
                "FREQ" => return frequency::decode(command, source),
                "FUNC" => return function::decode(command, source),
                "PHAS" => return phase::decode(command, source),
                "VOLT" => return voltage::decode(command, source),
                _ => {}
            }
        }
    }

    None
}
