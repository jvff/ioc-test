mod frequency;
mod function;
mod phase;
mod voltage;

use std::fmt;
use std::fmt::{Display, Formatter};

use super::super::request::ScpiRequest;
use super::super::str_extensions::StrExtensions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ScpiSourceSubsystem {
    source: usize,
    command: SourceCommand,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SourceCommand {
    FrequencyGet,
    PhaseGet,
    VoltageGet,
    VoltageOffsetGet,
    FunctionQuery,
    ArbitraryFunctionFileQuery,
    ArbitraryFunctionSampleRateGet,
    NoiseFunctionBandwidthGet,
    PrbsFunctionBitRateGet,
    PrbsFunctionPolynomialGet,
    PrbsFunctionTransitionGet,
    PulseFunctionLeadingEdgeTransitionGet,
    PulseFunctionTrailingEdgeTransitionGet,
    PulseFunctionPulseWidthGet,
    RampFunctionSymmetryGet,
    SquareFunctionDutyCycleGet,
}

impl Display for ScpiSourceSubsystem {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let source = self.source;

        match self.command {
            SourceCommand::FrequencyGet => {
                write!(formatter, "SOUR{}:FREQ?", source)
            }
            SourceCommand::PhaseGet => {
                write!(formatter, "SOUR{}:PHAS?", source)
            }
            SourceCommand::VoltageGet => {
                write!(formatter, "SOUR{}:VOLT?", source)
            }
            SourceCommand::VoltageOffsetGet => {
                write!(formatter, "SOUR{}:VOLT:OFFSet?", source)
            }
            SourceCommand::FunctionQuery => {
                write!(formatter, "SOUR{}:FUNC?", source)
            }
            SourceCommand::ArbitraryFunctionFileQuery => {
                write!(formatter, "SOUR{}:FUNC:ARB?", source)
            }
            SourceCommand::ArbitraryFunctionSampleRateGet => {
                write!(formatter, "SOUR{}:FUNC:ARB:SRAT?", source)
            }
            SourceCommand::NoiseFunctionBandwidthGet => {
                write!(formatter, "SOUR{}:FUNC:NOIS:BAND?", source)
            }
            SourceCommand::PrbsFunctionBitRateGet => {
                write!(formatter, "SOUR{}:FUNC:PRBS:BRAT?", source)
            }
            SourceCommand::PrbsFunctionPolynomialGet => {
                write!(formatter, "SOUR{}:FUNC:PRBS:DATA?", source)
            }
            SourceCommand::PrbsFunctionTransitionGet => {
                write!(formatter, "SOUR{}:FUNC:PRBS:TRAN?", source)
            }
            SourceCommand::PulseFunctionLeadingEdgeTransitionGet => {
                write!(formatter, "SOUR{}:FUNC:PULS:TRAN:LEAD?", source)
            }
            SourceCommand::PulseFunctionTrailingEdgeTransitionGet => {
                write!(formatter, "SOUR{}:FUNC:PULS:TRAN:TRA?", source)
            }
            SourceCommand::PulseFunctionPulseWidthGet => {
                write!(formatter, "SOUR{}:FUNC:PULS:WIDT?", source)
            }
            SourceCommand::RampFunctionSymmetryGet => {
                write!(formatter, "SOUR{}:FUNC:RAMP:SYMM?", source)
            }
            SourceCommand::SquareFunctionDutyCycleGet => {
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

            let decoded_command = match command.view_first_chars(4) {
                "FREQ" => frequency::decode(command),
                "FUNC" => function::decode(command),
                "PHAS" => phase::decode(command),
                "VOLT" => voltage::decode(command),
                _ => None,
            };

            if let Some(command) = decoded_command {
                return Some(ScpiSourceSubsystem { source, command });
            }
        }
    }

    None
}

pub type Subsystem = ScpiSourceSubsystem;
